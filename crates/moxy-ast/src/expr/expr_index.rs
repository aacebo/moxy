use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// An index expression: `a[0]`.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprIndex {
    pub attrs: Attributes,
    pub base: Box<Expr>,
    pub index: Delimited<Box<Expr>>,
}

impl From<ExprIndex> for Expr {
    fn from(value: ExprIndex) -> Self {
        Self::Index(value)
    }
}

impl Spanner for ExprIndex {
    fn span(&self) -> Span {
        self.attrs.span().join(self.index.span())
    }
}

impl ToTokens for ExprIndex {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.base.to_tokens(t);
        self.index.to_tokens(t);
    }
}
