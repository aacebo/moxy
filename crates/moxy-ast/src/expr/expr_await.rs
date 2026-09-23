use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// An await expression: `expr.await`.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprAwait {
    pub attrs: Attributes,
    pub base: Box<Expr>,
    pub dot: Token![.],
    pub await_keyword: Token![await],
}

impl From<ExprAwait> for Expr {
    fn from(value: ExprAwait) -> Self {
        Self::Await(value)
    }
}

impl Spanner for ExprAwait {
    fn span(&self) -> Span {
        self.attrs.span().join(self.await_keyword.span())
    }
}

impl ToTokens for ExprAwait {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.base.to_tokens(t);
        self.dot.to_tokens(t);
        self.await_keyword.to_tokens(t);
    }
}
