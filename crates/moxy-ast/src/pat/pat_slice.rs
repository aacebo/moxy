use moxy_token::token::punct::Comma;
use moxy_token::token::{Delim, ToTokens};
use moxy_token::{Span, TokenStream};

use crate::*;

#[doc = "A slice pattern, e.g. `[a, b, c]`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatSlice {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub elems: Punctuated<Pattern, Comma>,
}

impl ToTokens for PatSlice {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        let mut inner = TokenStream::new();
        self.elems.to_tokens(&mut inner);
        t.extend_one(moxy_token::TokenTree::Group(moxy_token::token::Group::new(
            moxy_token::token::Delim::Bracket,
            inner,
        )));
    }
}
