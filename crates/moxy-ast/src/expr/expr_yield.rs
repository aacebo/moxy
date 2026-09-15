use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A yield expression: `yield`, `yield expr`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprYield {
    pub attrs: Attributes,
    pub yield_keyword: Token![yield],
    pub expr: Option<Box<Expr>>,
}

impl From<ExprYield> for Expr {
    fn from(value: ExprYield) -> Self {
        Self::Yield(value)
    }
}

impl Spanner for ExprYield {
    fn span(&self) -> Span {
        let end = if let Some(e) = &self.expr {
            e.span()
        } else {
            self.yield_keyword.span()
        };

        self.attrs.span().join(end)
    }
}

impl ToTokens for ExprYield {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.yield_keyword.to_tokens(t);
        self.expr.to_tokens(t);
    }
}
