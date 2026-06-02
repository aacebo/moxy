mod expr_cast;
mod expr_reference;
mod expr_try;
mod expr_unary;

pub use expr_cast::*;
pub use expr_reference::*;
pub use expr_try::*;
pub use expr_unary::*;

use super::binary::ExprRange;
use super::{BinaryExpr, Expr};
use crate::Span;
use crate::ast::{Mutability, UnOp};
use crate::parse::{ParseError, ParseStream};
use crate::token::punct::And;
use crate::token::{ToTokens, TokenStream};

#[doc = "Unary prefix expressions (reference, unary op, cast, try-propagation)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum UnaryExpr {
    Reference(ExprReference),
    Unary(ExprUnary),
    Cast(ExprCast),
    Try(ExprTry),
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
        // Prefix range: `..b`, `..=b`, `..`.
        if stream.peek::<crate::token::punct::DotDot>().is_some() || stream.peek::<crate::token::punct::DotDotEq>().is_some() {
            use crate::ast::RangeLimits;
            let limits = stream.parse::<RangeLimits>()?;
            let end = super::binary::ExprRange::maybe_end(stream, allow_struct)?;
            return Ok(Expr::Binary(BinaryExpr::Range(ExprRange {
                span: Span::default(),
                attrs: Vec::new(),
                start: None,
                limits,
                end,
            })));
        }

        if stream.peek::<And>().is_some() {
            let _ = stream.parse::<And>()?;
            let mutability = stream.parse::<Mutability>()?;
            let expr = Box::new(UnaryExpr::parse_from(stream, allow_struct)?);
            return Ok(Expr::Unary(UnaryExpr::Reference(ExprReference {
                span: Span::default(),
                attrs: Vec::new(),
                mutability,
                expr,
            })));
        }

        if ExprUnary::is_prefix(stream) {
            let op = stream.parse::<UnOp>()?;
            let expr = Box::new(UnaryExpr::parse_from(stream, allow_struct)?);
            return Ok(Expr::Unary(UnaryExpr::Unary(ExprUnary {
                span: Span::default(),
                attrs: Vec::new(),
                op,
                expr,
            })));
        }

        let atom = super::primary::PrimaryExpr::parse_from(stream, allow_struct)?;
        super::postfix::PostfixExpr::parse_from(stream, atom)
    }
}
