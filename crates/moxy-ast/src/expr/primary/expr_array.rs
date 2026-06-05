use moxy_token::punct::Comma;
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

#[doc = "An array expression: `[a, b, c]`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprArray {
    pub attrs: Vec<Attribute>,
    pub elems: Delimited<Punctuated<super::super::Expr, Comma>>,
}

impl Spanner for ExprArray {
    fn span(&self) -> Span {
        self.elems.span()
    }
}

impl ToTokens for ExprArray {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.elems.to_tokens(t);
    }
}
