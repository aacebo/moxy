use crate::Token;
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// An array expression: `[a, b, c]`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprArray {
    pub attrs: Attributes,
    pub elems: Delimited<Punctuated<Expr, Token![,]>>,
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

impl ExprArray {
    pub fn into_primary_expr(self) -> super::PrimaryExpr {
        super::PrimaryExpr::from(self)
    }
}
