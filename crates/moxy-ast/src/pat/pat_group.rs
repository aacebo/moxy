use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A grouped pattern (used internally by the parser for grouping token trees).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatGroup {
    pub attrs: Attributes,
    pub pat: Box<Pattern>,
}

impl Spanner for PatGroup {
    fn span(&self) -> Span {
        let start = if let Some(a) = self.attrs.first() {
            a.span()
        } else {
            self.pat.span()
        };
        start.join(self.pat.span())
    }
}

impl ToTokens for PatGroup {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.pat.to_tokens(t);
    }
}

impl PatGroup {
    pub fn into_pattern(self) -> super::Pattern {
        super::Pattern::from(self)
    }
}
