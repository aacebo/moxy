mod expr_break;
mod expr_continue;
mod expr_return;
mod expr_yield;

pub use expr_break::*;
pub use expr_continue::*;
pub use expr_return::*;
pub use expr_yield::*;

use crate::token::{ToTokens, TokenStream};

#[doc = "Jump/control-flow expressions: return, break, continue, yield."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum JumpExpr {
    Return(ExprReturn),
    Break(ExprBreak),
    Continue(ExprContinue),
    Yield(ExprYield),
}

impl ToTokens for JumpExpr {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            JumpExpr::Return(v) => v.to_tokens(t),
            JumpExpr::Break(v) => v.to_tokens(t),
            JumpExpr::Continue(v) => v.to_tokens(t),
            JumpExpr::Yield(v) => v.to_tokens(t),
        }
    }
}

impl From<ExprReturn> for JumpExpr {
    fn from(v: ExprReturn) -> Self {
        JumpExpr::Return(v)
    }
}

impl From<ExprBreak> for JumpExpr {
    fn from(v: ExprBreak) -> Self {
        JumpExpr::Break(v)
    }
}

impl From<ExprContinue> for JumpExpr {
    fn from(v: ExprContinue) -> Self {
        JumpExpr::Continue(v)
    }
}

impl From<ExprYield> for JumpExpr {
    fn from(v: ExprYield) -> Self {
        JumpExpr::Yield(v)
    }
}
