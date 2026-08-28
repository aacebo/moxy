use moxy_token::Token;
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A reference pattern, e.g. `&x` or `&mut x`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatReference {
    pub attrs: Attributes,
    pub and: Token![&],
    pub mutability: Mutability,
    pub pat: Box<Pattern>,
}

impl Spanner for PatReference {
    fn span(&self) -> Span {
        self.attrs.span().join(self.pat.span())
    }
}

impl ToTokens for PatReference {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.and.to_tokens(t);
        self.mutability.to_tokens(t);
        self.pat.to_tokens(t);
    }
}

impl PatReference {
    pub fn into_pattern(self) -> super::Pattern {
        super::Pattern::from(self)
    }
}
