mod expr_break;
mod expr_continue;
mod expr_return;
mod expr_yield;

pub use expr_break::*;
pub use expr_continue::*;
pub use expr_return::*;
pub use expr_yield::*;
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::Attributes;

/// Jump/control-flow expressions: return, break, continue, yield.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum JumpExpr {
    Return(ExprReturn),
    Break(ExprBreak),
    Continue(ExprContinue),
    Yield(ExprYield),
}

impl JumpExpr {
    pub fn attrs(&self) -> &Attributes {
        match self {
            Self::Return(v) => &v.attrs,
            Self::Break(v) => &v.attrs,
            Self::Continue(v) => &v.attrs,
            Self::Yield(v) => &v.attrs,
        }
    }

    pub fn attrs_mut(&mut self) -> &mut Attributes {
        match self {
            Self::Return(v) => &mut v.attrs,
            Self::Break(v) => &mut v.attrs,
            Self::Continue(v) => &mut v.attrs,
            Self::Yield(v) => &mut v.attrs,
        }
    }

    pub fn is_return(&self) -> bool {
        matches!(self, Self::Return(_))
    }

    pub fn is_break(&self) -> bool {
        matches!(self, Self::Break(_))
    }

    pub fn is_continue(&self) -> bool {
        matches!(self, Self::Continue(_))
    }

    pub fn is_yield(&self) -> bool {
        matches!(self, Self::Yield(_))
    }

    pub fn as_return(&self) -> Option<&ExprReturn> {
        if let Self::Return(v) = self { Some(v) } else { None }
    }

    pub fn as_break(&self) -> Option<&ExprBreak> {
        if let Self::Break(v) = self { Some(v) } else { None }
    }

    pub fn as_continue(&self) -> Option<&ExprContinue> {
        if let Self::Continue(v) = self { Some(v) } else { None }
    }

    pub fn as_yield(&self) -> Option<&ExprYield> {
        if let Self::Yield(v) = self { Some(v) } else { None }
    }

    pub fn into_expr(self) -> super::Expr {
        super::Expr::from(self)
    }
}

impl Spanner for JumpExpr {
    fn span(&self) -> Span {
        match self {
            Self::Return(v) => v.span(),
            Self::Break(v) => v.span(),
            Self::Continue(v) => v.span(),
            Self::Yield(v) => v.span(),
        }
    }
}

impl ToTokens for JumpExpr {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            Self::Return(v) => v.to_tokens(t),
            Self::Break(v) => v.to_tokens(t),
            Self::Continue(v) => v.to_tokens(t),
            Self::Yield(v) => v.to_tokens(t),
        }
    }
}

impl From<ExprReturn> for JumpExpr {
    fn from(v: ExprReturn) -> Self {
        Self::Return(v)
    }
}

impl From<ExprBreak> for JumpExpr {
    fn from(v: ExprBreak) -> Self {
        Self::Break(v)
    }
}

impl From<ExprContinue> for JumpExpr {
    fn from(v: ExprContinue) -> Self {
        Self::Continue(v)
    }
}

impl From<ExprYield> for JumpExpr {
    fn from(v: ExprYield) -> Self {
        Self::Yield(v)
    }
}
