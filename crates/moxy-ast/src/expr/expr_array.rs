use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// An array expression: `[a, b, c]`.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprArray {
    pub attrs: Attributes,
    pub elems: Delimited<Punctuated<Expr, Token![,]>>,
}

impl From<ExprArray> for Expr {
    fn from(value: ExprArray) -> Self {
        Self::Array(value)
    }
}

impl Spanner for ExprArray {
    fn span(&self) -> Span {
        self.elems.span()
    }
}

impl ToTokens for ExprArray {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.elems.to_tokens(t);
    }
}
