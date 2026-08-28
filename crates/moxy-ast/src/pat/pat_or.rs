use moxy_token::Token;
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// An or-pattern, e.g. `A | B | C`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatOr {
    pub attrs: Attributes,
    pub cases: Punctuated<Pattern, Token![|]>,
}

impl Spanner for PatOr {
    fn span(&self) -> Span {
        let cases = match (self.cases.first(), self.cases.last()) {
            (Some(a), Some(b)) => a.span().join(b.span()),
            _ => Span::call_site(),
        };

        self.attrs.span().join(cases)
    }
}

impl ToTokens for PatOr {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.cases.to_tokens(t);
    }
}

impl PatOr {
    pub fn into_pattern(self) -> super::Pattern {
        super::Pattern::from(self)
    }
}
