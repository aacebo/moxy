use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A try block expression: `try { ... }`.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprTryBlock {
    pub attrs: Attributes,
    pub try_keyword: Token![try],
    pub block: StmtBlock,
}

impl From<ExprTryBlock> for Expr {
    fn from(value: ExprTryBlock) -> Self {
        Self::TryBlock(value)
    }
}

impl Spanner for ExprTryBlock {
    fn span(&self) -> Span {
        self.attrs.span().join(self.block.span())
    }
}

impl ToTokens for ExprTryBlock {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.try_keyword.to_tokens(t);
        self.block.to_tokens(t);
    }
}
