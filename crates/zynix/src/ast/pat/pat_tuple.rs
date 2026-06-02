use crate::ast::*;
use crate::token::punct::Comma;
use crate::token::{Delim, ToTokens};
use crate::{Span, TokenStream};

#[doc = "A tuple pattern, e.g. `(a, b, c)`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatTuple {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub elems: Punctuated<Pattern, Comma>,
}

impl ToTokens for PatTuple {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        let mut inner = TokenStream::new();
        self.elems.to_tokens(&mut inner);
        t.extend_one(crate::TokenTree::Group(crate::token::Group::new(
            crate::token::Delim::Paren,
            inner,
        )));
    }
}
