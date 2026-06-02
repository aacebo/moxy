use crate::ast::*;
use crate::token::ToTokens;
use crate::{Span, TokenStream};

#[doc = "A range pattern, e.g. `0..=255` or `'a'..'z'`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatRange {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub start: Option<Expr>,
    pub limits: RangeLimits,
    pub end: Option<Expr>,
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
