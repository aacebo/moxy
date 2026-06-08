mod expr_assign;
mod expr_assign_op;
mod expr_binary;
mod expr_range;
mod expr_type;

pub use expr_assign::*;
pub use expr_assign_op::*;
pub use expr_binary::*;
pub use expr_range::*;
pub use expr_type::*;
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::{DotDot, Eq};
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use super::unary::ExprCast;
use super::{Expr, UnaryExpr};
use crate::precedence::Precedence;
use crate::{AssignOp, BinOp, RangeLimits, Type};

/// Binary and assignment expressions.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum BinaryExpr {
    Binary(ExprBinary),
    Assign(ExprAssign),
    AssignOp(ExprAssignOp),
    Range(ExprRange),
    Type(ExprType),
}

impl Spanner for BinaryExpr {
    fn span(&self) -> Span {
        match self {
            BinaryExpr::Binary(v) => v.span(),
            BinaryExpr::Assign(v) => v.span(),
            BinaryExpr::AssignOp(v) => v.span(),
            BinaryExpr::Range(v) => v.span(),
            BinaryExpr::Type(v) => v.span(),
        }
    }
}

impl ToTokens for BinaryExpr {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            BinaryExpr::Binary(v) => v.to_tokens(t),
            BinaryExpr::Assign(v) => v.to_tokens(t),
            BinaryExpr::AssignOp(v) => v.to_tokens(t),
            BinaryExpr::Range(v) => v.to_tokens(t),
            BinaryExpr::Type(v) => v.to_tokens(t),
        }
    }
}

impl From<ExprBinary> for BinaryExpr {
    fn from(v: ExprBinary) -> Self {
        BinaryExpr::Binary(v)
    }
}

impl From<ExprAssign> for BinaryExpr {
    fn from(v: ExprAssign) -> Self {
        BinaryExpr::Assign(v)
    }
}

impl From<ExprAssignOp> for BinaryExpr {
    fn from(v: ExprAssignOp) -> Self {
        BinaryExpr::AssignOp(v)
    }
}

impl From<ExprRange> for BinaryExpr {
    fn from(v: ExprRange) -> Self {
        BinaryExpr::Range(v)
    }
}

impl From<ExprType> for BinaryExpr {
    fn from(v: ExprType) -> Self {
        BinaryExpr::Type(v)
    }
}

// Parser

impl BinaryExpr {
    pub fn parse_from(stream: &mut ParseStream, mut lhs: Expr, min: Precedence, allow_struct: bool) -> Result<Expr, ParseError> {
        loop {
            if Precedence::Cast >= min && stream.peek::<moxy_token::keyword::As>() {
                let as_keyword = stream.parse::<moxy_token::keyword::As>()?;
                let ty = Box::new(stream.parse::<Type>()?);

                lhs = Expr::Unary(UnaryExpr::Cast(ExprCast {
                    attrs: Vec::new(),
                    expr: Box::new(lhs),
                    as_keyword,
                    ty,
                }));

                continue;
            }

            if min == Precedence::Min {
                if stream.peek::<Eq>() {
                    let eq = stream.parse::<Eq>()?;
                    let right = Box::new(super::parse_expr(stream, allow_struct)?);

                    lhs = Expr::Binary(BinaryExpr::Assign(ExprAssign {
                        attrs: Vec::new(),
                        left: Box::new(lhs),
                        eq,
                        right,
                    }));

                    continue;
                }

                if let Ok(op) = stream.parse::<AssignOp>() {
                    let right = Box::new(super::parse_expr(stream, allow_struct)?);

                    lhs = Expr::Binary(BinaryExpr::AssignOp(ExprAssignOp {
                        attrs: Vec::new(),
                        left: Box::new(lhs),
                        op,
                        right,
                    }));

                    continue;
                }
            }

            // Range with a left operand: `a..b`, `a..=b`, `a..` (Precedence::Range).
            if Precedence::Range >= min && (stream.peek::<DotDot>() || stream.peek::<moxy_token::punct::DotDotEq>()) {
                let limits = stream.parse::<RangeLimits>()?;
                let end = ExprRange::maybe_end(stream, allow_struct)?;

                lhs = Expr::Binary(BinaryExpr::Range(ExprRange {
                    attrs: Vec::new(),
                    start: Some(Box::new(lhs)),
                    limits,
                    end,
                }));

                continue;
            }

            match stream.fork().parse::<BinOp>() {
                Ok(op) if Precedence::of(&op) >= min => {
                    let prec = Precedence::of(&op);
                    let _ = stream.parse::<BinOp>()?;
                    let mut rhs = super::unary::UnaryExpr::parse_from(stream, allow_struct)?;

                    rhs = BinaryExpr::parse_from(stream, rhs, prec.next(), allow_struct)?;
                    lhs = Expr::Binary(BinaryExpr::Binary(ExprBinary {
                        attrs: Vec::new(),
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
