use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A for loop expression: `for pat in expr { ... }`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprForLoop {
    pub attrs: Attributes,
    pub label: Option<Label>,
    pub for_keyword: Token![for],
    pub pat: Box<Pattern>,
    pub in_keyword: Token![in],
    pub expr: Box<Expr>,
    pub body: StmtBlock,
}

impl From<ExprForLoop> for Expr {
    fn from(value: ExprForLoop) -> Self {
        Self::ForLoop(value)
    }
}

impl Spanner for ExprForLoop {
    fn span(&self) -> Span {
        self.attrs.span().join(self.body.span())
    }
}

impl ToTokens for ExprForLoop {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.label.to_tokens(t);
        self.for_keyword.to_tokens(t);
        self.pat.to_tokens(t);
        self.in_keyword.to_tokens(t);
        self.expr.to_tokens(t);
        self.body.to_tokens(t);
    }
}
