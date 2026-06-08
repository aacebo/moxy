use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A grouped pattern (used internally by the parser for grouping token trees).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatGroup {
    pub attrs: Vec<Attribute>,
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
        for a in &self.attrs {
            a.to_tokens(t);
        }

        self.pat.to_tokens(t);
    }
}
