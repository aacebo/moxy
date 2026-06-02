use moxy_token::token::{Delim, ToTokens};
use moxy_token::{Span, TokenStream};

use crate::*;

#[doc = "A parenthesized pattern, e.g. `(A | B)`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatParen {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub pat: Box<Pattern>,
}

impl ToTokens for PatParen {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        let mut inner = TokenStream::new();
        self.pat.to_tokens(&mut inner);
        t.extend_one(moxy_token::TokenTree::Group(moxy_token::token::Group::new(
            moxy_token::token::Delim::Paren,
            inner,
        )));
    }
}
