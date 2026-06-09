use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A parenthesized pattern, e.g. `(A | B)`.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatParen {
    pub attrs: Vec<Attribute>,
    pub content: Delimited<Box<Pattern>>,
}

impl Spanner for PatParen {
    fn span(&self) -> Span {
        self.content.span()
    }
}

impl ToTokens for PatParen {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.content.to_tokens(t);
    }
}

impl PatParen {
    pub fn into_pattern(self) -> super::Pattern {
        super::Pattern::from(self)
    }
}
