use moxy_token::punct::Comma;
use moxy_token::{Bracket, Span, ToTokens, TokenStream};

use crate::*;

#[doc = "An array expression: `[a, b, c]`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprArray {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub bracket: Bracket,
    pub elems: Punctuated<super::super::Expr, Comma>,
}

impl ToTokens for ExprArray {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        let mut inner = TokenStream::new();
        self.elems.to_tokens(&mut inner);
        self.bracket.surround(t, inner);
    }
}
