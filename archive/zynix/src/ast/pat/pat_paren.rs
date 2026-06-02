use crate::ast::*;
use crate::token::{Delim, ToTokens};
use crate::{Span, TokenStream};

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
        t.extend_one(crate::TokenTree::Group(crate::token::Group::new(
            crate::token::Delim::Paren,
            inner,
        )));
    }
}
