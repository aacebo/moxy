use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

#[doc = "A range pattern, e.g. `0..=255` or `'a'..'z'`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatRange {
    pub attrs: Vec<Attribute>,
    pub start: Option<Expr>,
    pub limits: RangeLimits,
    pub end: Option<Expr>,
}

impl Spanner for PatRange {
    fn span(&self) -> Span {
        let start = if let Some(a) = self.attrs.first() {
            a.span()
        } else if let Some(s) = &self.start {
            s.span()
        } else {
            self.limits.span()
        };
        let end = if let Some(e) = &self.end {
            e.span()
        } else {
            self.limits.span()
        };
        start.join(end)
    }
}

impl ToTokens for PatRange {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }

        if let Some(s) = &self.start {
            s.to_tokens(t);
        }

        self.limits.to_tokens(t);

        if let Some(e) = &self.end {
            e.to_tokens(t);
        }
    }
}
