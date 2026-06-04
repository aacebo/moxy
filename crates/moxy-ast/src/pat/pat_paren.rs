use moxy_token::{Span, ToTokens, TokenStream};

use crate::*;

#[doc = "A parenthesized pattern, e.g. `(A | B)`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatParen {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub paren: Delimited<Box<Pattern>>,
}

impl ToTokens for PatParen {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.paren.to_tokens(t);
    }
}
