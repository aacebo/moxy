use moxy_token::Token;
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A slice pattern, e.g. `[a, b, c]`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatSlice {
    pub attrs: Attributes,
    pub elems: Delimited<Punctuated<Pattern, Token![,]>>,
}

impl Spanner for PatSlice {
    fn span(&self) -> Span {
        self.elems.span()
    }
}

impl ToTokens for PatSlice {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.elems.to_tokens(t);
    }
}

impl PatSlice {
    pub fn into_pattern(self) -> super::Pattern {
        super::Pattern::from(self)
    }
}
