use moxy_token::punct::Or as OrPunct;
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// An or-pattern, e.g. `A | B | C`.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatOr {
    pub attrs: Vec<Attribute>,
    pub cases: Punctuated<Pattern, OrPunct>,
}

impl Spanner for PatOr {
    fn span(&self) -> Span {
        let start = if let Some(a) = self.attrs.first() {
            a.span()
        } else if let Some(c) = self.cases.first() {
            c.span()
        } else {
            Span::call_site()
        };
        let end = self.cases.last().map(|c| c.span()).unwrap_or(start);
        start.join(end)
    }
}

impl ToTokens for PatOr {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }

        self.cases.to_tokens(t);
    }
}
