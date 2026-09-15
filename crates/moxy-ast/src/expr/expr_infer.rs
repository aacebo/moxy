use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// `let _ = ...`
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprInfer {
    pub attrs: Attributes,
    pub underscore: Token![_],
}

impl From<ExprInfer> for Expr {
    fn from(value: ExprInfer) -> Self {
        Self::Infer(value)
    }
}

impl Spanner for ExprInfer {
    fn span(&self) -> Span {
        self.attrs.span().join(self.underscore.span())
    }
}

impl ToTokens for ExprInfer {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.underscore.to_tokens(t);
    }
}
