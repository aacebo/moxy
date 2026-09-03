use moxy_token::Token;

use crate::{Expr, ParseError, Parser, Precedence, Type};

impl<'a> Parser<'a> {
    pub fn parse_binary(&mut self, mut lhs: Expr, min: Precedence, allow_struct: bool) -> Result<Expr, ParseError> {
        loop {
            if Precedence::Cast >= min && self.peek::<Token![as]>() {
                let as_keyword = self.parse::<Token![as]>()?;
                let ty = Box::new(self.parse::<Type>()?);

                lhs = Expr::Unary(UnaryExpr::Cast(ExprCast {
                    attrs: Attributes::default(),
                    expr: Box::new(lhs),
                    as_keyword,
                    ty,
                }));

                continue;
            }

            if min == Precedence::Min {
                if self.peek::<Token![=]>() {
                    let eq = self.parse::<Token![=]>()?;
                    let right = Box::new(super::parse_expr(stream, allow_struct)?);

                    lhs = Expr::Binary(Self::Assign(ExprAssign {
                        attrs: Attributes::default(),
                        left: Box::new(lhs),
                        eq,
                        right,
                    }));

                    continue;
                }

                if self.peek::<AssignOp>() {
                    let op = self.parse::<AssignOp>()?;
                    let right = Box::new(super::parse_expr(stream, allow_struct)?);

                    lhs = Expr::Binary(Self::AssignOp(ExprAssignOp {
                        attrs: Attributes::default(),
                        left: Box::new(lhs),
                        op,
                        right,
                    }));

                    continue;
                }
            }

            // Range with a left operand: `a..b`, `a..=b`, `a..` (Precedence::Range).
            if Precedence::Range >= min && (self.peek::<Token![..]>() || self.peek::<Token![..=]>()) {
                let limits = self.parse::<RangeLimits>()?;
                let end = ExprRange::maybe_end(stream, allow_struct)?;

                lhs = Expr::Binary(Self::Range(ExprRange {
                    attrs: Attributes::default(),
                    start: Some(Box::new(lhs)),
                    limits,
                    end,
                }));

                continue;
            }

            match Precedence::peek(stream) {
                Some(prec) if prec >= min => {
                    let op = self.parse::<BinOp>()?;
                    let mut rhs = super::unary::UnaryExpr::parse_from(stream, allow_struct)?;

                    rhs = Self::parse_from(stream, rhs, prec.next(), allow_struct)?;
                    lhs = Expr::Binary(Self::Binary(ExprBinary {
                        attrs: Attributes::default(),
                        left: Box::new(lhs),
                        op,
                        right: Box::new(rhs),
                    }));
                }
                _ => break,
            }
        }

        Ok(lhs)
    }
}
