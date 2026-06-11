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
use crate::{AssignOp, Attributes, BinOp, RangeLimits, Type};

/// Binary and assignment expressions.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum BinaryExpr {
    Binary(ExprBinary),
    Assign(ExprAssign),
    AssignOp(ExprAssignOp),
    Range(ExprRange),
    Type(ExprType),
}

impl BinaryExpr {
    pub fn attrs_mut(&mut self) -> &mut Attributes {
        match self {
            Self::Binary(v) => &mut v.attrs,
            Self::Assign(v) => &mut v.attrs,
            Self::AssignOp(v) => &mut v.attrs,
            Self::Range(v) => &mut v.attrs,
            Self::Type(v) => &mut v.attrs,
        }
    }

    pub fn is_binary(&self) -> bool {
        matches!(self, Self::Binary(_))
    }

    pub fn is_assign(&self) -> bool {
        matches!(self, Self::Assign(_))
    }

    pub fn is_assign_op(&self) -> bool {
        matches!(self, Self::AssignOp(_))
    }

    pub fn is_range(&self) -> bool {
        matches!(self, Self::Range(_))
    }

    pub fn is_type(&self) -> bool {
        matches!(self, Self::Type(_))
    }

    pub fn as_binary(&self) -> Option<&ExprBinary> {
        if let Self::Binary(v) = self { Some(v) } else { None }
    }

    pub fn as_assign(&self) -> Option<&ExprAssign> {
        if let Self::Assign(v) = self { Some(v) } else { None }
    }

    pub fn as_assign_op(&self) -> Option<&ExprAssignOp> {
        if let Self::AssignOp(v) = self { Some(v) } else { None }
    }

    pub fn as_range(&self) -> Option<&ExprRange> {
        if let Self::Range(v) = self { Some(v) } else { None }
    }

    pub fn as_type(&self) -> Option<&ExprType> {
        if let Self::Type(v) = self { Some(v) } else { None }
    }

    pub fn into_expr(self) -> super::Expr {
        super::Expr::from(self)
    }
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
                    attrs: Attributes::default(),
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
                        attrs: Attributes::default(),
                        left: Box::new(lhs),
                        eq,
                        right,
                    }));

                    continue;
                }

                if let Ok(op) = stream.parse::<AssignOp>() {
                    let right = Box::new(super::parse_expr(stream, allow_struct)?);

                    lhs = Expr::Binary(BinaryExpr::AssignOp(ExprAssignOp {
                        attrs: Attributes::default(),
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
                    attrs: Attributes::default(),
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
