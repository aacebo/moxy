use moxy_token::punct::And;
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A reference pattern, e.g. `&x` or `&mut x`.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatReference {
    pub attrs: Vec<Attribute>,
    pub and: And,
    pub mutability: Mutability,
    pub pat: Box<Pattern>,
}

impl Spanner for PatReference {
    fn span(&self) -> Span {
        let start = if let Some(a) = self.attrs.first() {
            a.span()
        } else {
            self.and.span()
        };
        start.join(self.pat.span())
    }
}

impl ToTokens for PatReference {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }

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
