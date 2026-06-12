use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A range pattern, e.g. `0..=255` or `'a'..'z'`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatRange {
    pub attrs: Attributes,
    pub start: Option<Expr>,
    pub limits: RangeLimits,
    pub end: Option<Expr>,
}

impl Spanner for PatRange {
    fn span(&self) -> Span {
        let end = if let Some(e) = &self.end {
            e.span()
        } else {
            self.limits.span()
        };
        self.attrs.span().join(end)
    }
}

impl ToTokens for PatRange {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);

        if let Some(s) = &self.start {
            s.to_tokens(t);
        }

        self.limits.to_tokens(t);

        if let Some(e) = &self.end {
            e.to_tokens(t);
        }
    }
}

impl PatRange {
    pub fn into_pattern(self) -> super::Pattern {
        super::Pattern::from(self)
    }
}
