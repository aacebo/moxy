use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// An unsafe block expression: `unsafe { ... }`.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprUnsafe {
    pub attrs: Attributes,
    pub unsafe_keyword: Token![unsafe],
    pub block: StmtBlock,
}

impl From<ExprUnsafe> for Expr {
    fn from(value: ExprUnsafe) -> Self {
        Self::Unsafe(value)
    }
}

impl Spanner for ExprUnsafe {
    fn span(&self) -> Span {
        self.attrs.span().join(self.block.span())
    }
}

impl ToTokens for ExprUnsafe {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.unsafe_keyword.to_tokens(t);
        self.block.to_tokens(t);
    }
}
