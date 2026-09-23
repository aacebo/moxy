use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A literal expression: `1`, `"hello"`, `true`.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprLit {
    pub attrs: Attributes,
    pub lit: Lit,
}

impl From<ExprLit> for Expr {
    fn from(value: ExprLit) -> Self {
        Self::Lit(value)
    }
}

impl Spanner for ExprLit {
    fn span(&self) -> Span {
        self.attrs.span().join(self.lit.span())
    }
}

impl ToTokens for ExprLit {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.lit.to_tokens(t);
    }
}
