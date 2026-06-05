use moxy_token::punct::Comma;
use moxy_token::{Span, ToTokens, TokenStream};

use crate::*;

#[doc = "A tuple expression: `(a, b, c)`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprTuple {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub elems: Delimited<Punctuated<super::super::Expr, Comma>>,
}

impl ToTokens for ExprTuple {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.elems.to_tokens(t);
    }
}
