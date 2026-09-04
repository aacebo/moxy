mod expr_cast;
mod expr_reference;
mod expr_try;
mod expr_unary;

use crate::{ParseError, Parser, Token};

pub use expr_cast::*;
pub use expr_reference::*;
pub use expr_try::*;
pub use expr_unary::*;
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use super::binary::ExprRange;
use super::{BinaryExpr, Expr};
use crate::{Attributes, Mutability, UnOp};

/// Unary prefix expressions (reference, unary op, cast, try-propagation).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum UnaryExpr {
    Reference(ExprReference),
    Unary(ExprUnary),
    Cast(ExprCast),
    Try(ExprTry),
}

impl UnaryExpr {
    pub fn attrs(&self) -> &Attributes {
        match self {
            Self::Reference(v) => &v.attrs,
            Self::Unary(v) => &v.attrs,
            Self::Cast(v) => &v.attrs,
            Self::Try(v) => &v.attrs,
        }
    }

    pub fn attrs_mut(&mut self) -> &mut Attributes {
        match self {
            Self::Reference(v) => &mut v.attrs,
            Self::Unary(v) => &mut v.attrs,
            Self::Cast(v) => &mut v.attrs,
            Self::Try(v) => &mut v.attrs,
        }
    }

    pub fn is_reference(&self) -> bool {
        matches!(self, Self::Reference(_))
    }

    pub fn is_unary(&self) -> bool {
        matches!(self, Self::Unary(_))
    }

    pub fn is_cast(&self) -> bool {
        matches!(self, Self::Cast(_))
    }

    pub fn is_try(&self) -> bool {
        matches!(self, Self::Try(_))
    }

    pub fn as_reference(&self) -> Option<&ExprReference> {
        if let Self::Reference(v) = self { Some(v) } else { None }
    }

    pub fn as_unary(&self) -> Option<&ExprUnary> {
        if let Self::Unary(v) = self { Some(v) } else { None }
    }

    pub fn as_cast(&self) -> Option<&ExprCast> {
        if let Self::Cast(v) = self { Some(v) } else { None }
    }

    pub fn as_try(&self) -> Option<&ExprTry> {
        if let Self::Try(v) = self { Some(v) } else { None }
    }

    pub fn into_expr(self) -> super::Expr {
        super::Expr::from(self)
    }
}

impl Spanner for UnaryExpr {
    fn span(&self) -> Span {
        match self {
            Self::Reference(v) => v.span(),
            Self::Unary(v) => v.span(),
            Self::Cast(v) => v.span(),
            Self::Try(v) => v.span(),
        }
    }
}

impl ToTokens for UnaryExpr {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            Self::Reference(v) => v.to_tokens(t),
            Self::Unary(v) => v.to_tokens(t),
            Self::Cast(v) => v.to_tokens(t),
            Self::Try(v) => v.to_tokens(t),
        }
    }
}

impl From<ExprReference> for UnaryExpr {
    fn from(v: ExprReference) -> Self {
        Self::Reference(v)
    }
}

impl From<ExprUnary> for UnaryExpr {
    fn from(v: ExprUnary) -> Self {
        Self::Unary(v)
    }
}

impl From<ExprCast> for UnaryExpr {
    fn from(v: ExprCast) -> Self {
        Self::Cast(v)
    }
}

impl From<ExprTry> for UnaryExpr {
    fn from(v: ExprTry) -> Self {
        Self::Try(v)
    }
}

// Parser

impl UnaryExpr {
    pub fn parse_from(parser: &Parser, allow_struct: bool) -> Result<Expr, ParseError> {
        // Leading outer attributes apply to whichever expression node we build here.
        let attrs = parser.parse::<Attributes>()?;

        // Prefix range: `..b`, `..=b`, `..`.
        if parser.peek::<Token![..]>() || parser.peek::<Token![..=]>() {
            use crate::RangeLimits;
            let limits = parser.parse::<RangeLimits>()?;
            let end = super::binary::ExprRange::maybe_end(parser, allow_struct)?;
            return Ok(Expr::Binary(BinaryExpr::Range(ExprRange {
                attrs,
                start: None,
                limits,
                end,
            })));
        }

        if parser.peek::<Token![&]>() {
            let and_punct = parser.parse::<Token![&]>()?;
            let mutability = parser.parse::<Mutability>()?;
            let expr = Box::new(Self::parse_from(parser, allow_struct)?);
            return Ok(Expr::Unary(Self::Reference(ExprReference {
                attrs,
                and_punct,
                mutability,
                expr,
            })));
        }

        if ExprUnary::is_prefix(parser) {
            let op = parser.parse::<UnOp>()?;
            let expr = Box::new(Self::parse_from(parser, allow_struct)?);
            return Ok(Expr::Unary(Self::Unary(ExprUnary { attrs, op, expr })));
        }

        let atom = super::primary::PrimaryExpr::parse_from(parser, allow_struct, attrs)?;
        super::postfix::PostfixExpr::parse_from(parser, atom)
    }
}
