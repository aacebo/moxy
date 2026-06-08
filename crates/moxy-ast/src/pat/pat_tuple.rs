use moxy_token::punct::Comma;
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A tuple pattern, e.g. `(a, b, c)`.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatTuple {
    pub attrs: Vec<Attribute>,
    pub elems: Delimited<Punctuated<Pattern, Comma>>,
}

impl Spanner for PatTuple {
    fn span(&self) -> Span {
        self.elems.span()
    }
}

impl ToTokens for PatTuple {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.elems.to_tokens(t);
    }
}
