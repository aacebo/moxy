use moxy_token::punct::Comma;
use moxy_token::{Paren, Span, ToTokens, TokenStream};

use crate::*;

#[doc = "A tuple expression: `(a, b, c)`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprTuple {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub paren: Paren,
    pub elems: Punctuated<super::super::Expr, Comma>,
}

impl ToTokens for ExprTuple {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        let mut inner = TokenStream::new();
        self.elems.to_tokens(&mut inner);
        self.paren.surround(t, inner);
    }
}
