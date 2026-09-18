use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A while loop expression: `while cond { ... }`, `while let pat = expr { ... }`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprWhile {
    pub attrs: Attributes,
    pub label: Option<Label>,
    pub while_keyword: Token![while],
    pub cond: Box<Expr>,
    pub body: StmtBlock,
}

impl From<ExprWhile> for Expr {
    fn from(value: ExprWhile) -> Self {
        Self::While(value)
    }
}

impl Spanner for ExprWhile {
    fn span(&self) -> Span {
        self.attrs.span().join(self.body.span())
    }
}

impl ToTokens for ExprWhile {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.label.to_tokens(t);
        self.while_keyword.to_tokens(t);
        self.cond.to_tokens(t);
        self.body.to_tokens(t);
    }
}
