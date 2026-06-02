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

use super::unary::ExprCast;
use super::{Expr, UnaryExpr};
use crate::Span;
use crate::ast::precedence::Precedence;
use crate::ast::{AssignOp, BinOp, RangeLimits, Type};
use crate::parse::{ParseError, ParseStream};
use crate::token::punct::{DotDot, Eq};
use crate::token::{ToTokens, TokenStream};

#[doc = "Binary and assignment expressions."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum BinaryExpr {
    Binary(ExprBinary),
    Assign(ExprAssign),
    AssignOp(ExprAssignOp),
    Range(ExprRange),
    Type(ExprType),
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
            if Precedence::Cast >= min && stream.peek::<crate::token::keyword::As>().is_some() {
                let _ = stream.parse::<crate::token::keyword::As>()?;
                let ty = Box::new(stream.parse::<Type>()?);
                lhs = Expr::Unary(UnaryExpr::Cast(ExprCast {
                    span: Span::default(),
                    attrs: Vec::new(),
                    expr: Box::new(lhs),
                    ty,
                }));
                continue;
            }

            if min == Precedence::Min {
                if stream.peek::<Eq>().is_some() {
                    let _ = stream.parse::<Eq>()?;
                    let right = Box::new(super::parse_expr(stream, allow_struct)?);
                    lhs = Expr::Binary(BinaryExpr::Assign(ExprAssign {
                        span: Span::default(),
                        attrs: Vec::new(),
                        left: Box::new(lhs),
                        right,
                    }));
                    continue;
                }

                if let Some(op) = stream.peek::<AssignOp>() {
                    let _ = stream.parse::<AssignOp>()?;
                    let right = Box::new(super::parse_expr(stream, allow_struct)?);
                    lhs = Expr::Binary(BinaryExpr::AssignOp(ExprAssignOp {
                        span: Span::default(),
                        attrs: Vec::new(),
                        left: Box::new(lhs),
                        op,
                        right,
                    }));
                    continue;
                }
            }

            // Range with a left operand: `a..b`, `a..=b`, `a..` (Precedence::Range).
            if Precedence::Range >= min
                && (stream.peek::<DotDot>().is_some() || stream.peek::<crate::token::punct::DotDotEq>().is_some())
            {
                let limits = stream.parse::<RangeLimits>()?;
                let end = ExprRange::maybe_end(stream, allow_struct)?;
                lhs = Expr::Binary(BinaryExpr::Range(ExprRange {
                    span: Span::default(),
                    attrs: Vec::new(),
                    start: Some(Box::new(lhs)),
                    limits,
                    end,
                }));
                continue;
            }

            match stream.peek::<BinOp>() {
                Some(op) if Precedence::of(&op) >= min => {
                    let prec = Precedence::of(&op);
                    let _ = stream.parse::<BinOp>()?;
                    let mut rhs = super::unary::UnaryExpr::parse_from(stream, allow_struct)?;
                    rhs = BinaryExpr::parse_from(stream, rhs, prec.next(), allow_struct)?;
                    lhs = Expr::Binary(BinaryExpr::Binary(ExprBinary {
                        span: Span::default(),
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
