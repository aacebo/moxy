use moxy_token::keyword::Yield;
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A yield expression: `yield`, `yield expr`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprYield {
    pub attrs: Vec<Attribute>,
    pub yield_keyword: Yield,
    pub expr: Option<Box<Expr>>,
}

impl Spanner for ExprYield {
    fn span(&self) -> Span {
        let start = if let Some(a) = self.attrs.first() {
            a.span()
        } else {
            self.yield_keyword.span()
        };
        let end = if let Some(e) = &self.expr {
            e.span()
        } else {
            self.yield_keyword.span()
        };
        start.join(end)
    }
}

impl ToTokens for ExprYield {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.yield_keyword.to_tokens(t);

        if let Some(e) = &self.expr {
            e.to_tokens(t);
        }
    }
}

impl ExprYield {
    pub fn into_jump_expr(self) -> super::JumpExpr {
        super::JumpExpr::from(self)
    }
}
