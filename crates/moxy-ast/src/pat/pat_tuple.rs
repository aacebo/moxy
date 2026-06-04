use moxy_token::punct::Comma;
use moxy_token::{Span, ToTokens, TokenStream};

use crate::*;

#[doc = "A tuple pattern, e.g. `(a, b, c)`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatTuple {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub paren: Delimited<Punctuated<Pattern, Comma>>,
}

impl ToTokens for PatTuple {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.paren.to_tokens(t);
    }
}
