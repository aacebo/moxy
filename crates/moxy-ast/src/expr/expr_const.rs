use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A const block expression: `const { ... }`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprConst {
    pub attrs: Attributes,
    pub const_keyword: Token![const],
    pub block: StmtBlock,
}

impl From<ExprConst> for Expr {
    fn from(value: ExprConst) -> Self {
        Self::Const(value)
    }
}

impl Spanner for ExprConst {
    fn span(&self) -> Span {
        self.attrs.span().join(self.block.span())
    }
}

impl ToTokens for ExprConst {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.const_keyword.to_tokens(t);
        self.block.to_tokens(t);
    }
}
