use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A loop expression: `loop { ... }`, `'label: loop { ... }`.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprLoop {
    pub attrs: Attributes,
    pub label: Option<Label>,
    pub loop_keyword: Token![loop],
    pub body: StmtBlock,
}

impl From<ExprLoop> for Expr {
    fn from(value: ExprLoop) -> Self {
        Self::Loop(value)
    }
}

impl Spanner for ExprLoop {
    fn span(&self) -> Span {
        self.attrs.span().join(self.body.span())
    }
}

impl ToTokens for ExprLoop {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.label.to_tokens(t);
        self.loop_keyword.to_tokens(t);
        self.body.to_tokens(t);
    }
}
