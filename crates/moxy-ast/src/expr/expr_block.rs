use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A block expression: `{ ... }`.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprBlock {
    pub attrs: Attributes,
    pub label: Option<Label>,
    pub block: StmtBlock,
}

impl From<ExprBlock> for Expr {
    fn from(value: ExprBlock) -> Self {
        Self::Block(value)
    }
}

impl Spanner for ExprBlock {
    fn span(&self) -> Span {
        self.attrs.span().join(self.block.span())
    }
}

impl ToTokens for ExprBlock {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.label.to_tokens(t);
        self.block.to_tokens(t);
    }
}
