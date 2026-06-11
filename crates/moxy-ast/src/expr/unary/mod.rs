mod expr_cast;
mod expr_reference;
mod expr_try;
mod expr_unary;

pub use expr_cast::*;
pub use expr_reference::*;
pub use expr_try::*;
pub use expr_unary::*;
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::And;
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
            UnaryExpr::Reference(v) => v.span(),
            UnaryExpr::Unary(v) => v.span(),
            UnaryExpr::Cast(v) => v.span(),
            UnaryExpr::Try(v) => v.span(),
        }
    }
}

impl ToTokens for UnaryExpr {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            UnaryExpr::Reference(v) => v.to_tokens(t),
            UnaryExpr::Unary(v) => v.to_tokens(t),
            UnaryExpr::Cast(v) => v.to_tokens(t),
            UnaryExpr::Try(v) => v.to_tokens(t),
        }
    }
}

impl From<ExprReference> for UnaryExpr {
    fn from(v: ExprReference) -> Self {
        UnaryExpr::Reference(v)
    }
}

impl From<ExprUnary> for UnaryExpr {
    fn from(v: ExprUnary) -> Self {
        UnaryExpr::Unary(v)
    }
}

impl From<ExprCast> for UnaryExpr {
    fn from(v: ExprCast) -> Self {
        UnaryExpr::Cast(v)
    }
}

impl From<ExprTry> for UnaryExpr {
    fn from(v: ExprTry) -> Self {
        UnaryExpr::Try(v)
    }
}

// Parser

impl UnaryExpr {
    pub fn parse_from(stream: &mut ParseStream, allow_struct: bool) -> Result<Expr, ParseError> {
        // Leading outer attributes apply to whichever expression node we build here.
        let attrs = stream.parse::<Attributes>()?;

        // Prefix range: `..b`, `..=b`, `..`.
        if stream.peek::<moxy_token::punct::DotDot>() || stream.peek::<moxy_token::punct::DotDotEq>() {
            use crate::RangeLimits;
            let limits = stream.parse::<RangeLimits>()?;
            let end = super::binary::ExprRange::maybe_end(stream, allow_struct)?;
            return Ok(Expr::Binary(BinaryExpr::Range(ExprRange {
                attrs,
                start: None,
                limits,
                end,
            })));
        }

        if stream.peek::<And>() {
            let and_punct = stream.parse::<And>()?;
            let mutability = stream.parse::<Mutability>()?;
            let expr = Box::new(UnaryExpr::parse_from(stream, allow_struct)?);
            return Ok(Expr::Unary(UnaryExpr::Reference(ExprReference {
                attrs,
                and_punct,
                mutability,
                expr,
            })));
        }

        if ExprUnary::is_prefix(stream) {
            let op = stream.parse::<UnOp>()?;
            let expr = Box::new(UnaryExpr::parse_from(stream, allow_struct)?);
            return Ok(Expr::Unary(UnaryExpr::Unary(ExprUnary { attrs, op, expr })));
        }

        let atom = super::primary::PrimaryExpr::parse_from(stream, allow_struct)?;
        let mut expr = super::postfix::PostfixExpr::parse_from(stream, atom)?;

        // Attach the leading attributes to the parsed atom/postfix node.
        if !attrs.is_empty()
            && let Some(slot) = expr.attrs_mut()
        {
            *slot = attrs;
        }

        Ok(expr)
    }
}
