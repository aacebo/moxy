use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// An async block expression: `async { ... }`, `async move { ... }`.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprAsync {
    pub attrs: Attributes,
    pub async_keyword: Token![async],
    pub move_keyword: Option<Token![move]>,
    pub block: StmtBlock,
}

impl From<ExprAsync> for Expr {
    fn from(value: ExprAsync) -> Self {
        Self::Async(value)
    }
}

impl Spanner for ExprAsync {
    fn span(&self) -> Span {
        self.attrs.span().join(self.block.span())
    }
}

impl ToTokens for ExprAsync {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.async_keyword.to_tokens(t);
        self.move_keyword.to_tokens(t);
        self.block.to_tokens(t);
    }
}
