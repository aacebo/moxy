use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A range expression: `0..10`, `a..=b`, `..`, `a..`, `..b`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprRange {
    pub attrs: Attributes,
    pub start: Option<Box<Expr>>,
    pub limits: RangeLimits,
    pub end: Option<Box<Expr>>,
}

impl From<ExprRange> for Expr {
    fn from(value: ExprRange) -> Self {
        Self::Range(value)
    }
}

impl Spanner for ExprRange {
    fn span(&self) -> Span {
        let end = if let Some(e) = &self.end {
            e.span()
        } else {
            self.limits.span()
        };

        self.attrs.span().join(end)
    }
}

impl ToTokens for ExprRange {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.start.to_tokens(t);
        self.limits.to_tokens(t);
        self.end.to_tokens(t);
    }
}
