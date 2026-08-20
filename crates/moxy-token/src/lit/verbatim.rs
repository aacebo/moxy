use crate::lit::Lit;
use crate::{Span, Spanner};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(into = "String"))]
pub struct LitVerbatim {
    repr: Box<str>,
    span: Span,
}

impl LitVerbatim {
    #[inline]
    pub fn new(repr: &str, span: Span) -> Self {
        Self { repr: repr.into(), span }
    }

    #[inline]
    pub fn repr(&self) -> &str {
        &self.repr
    }

    #[inline]
    pub fn span(&self) -> Span {
        self.span
    }

    #[inline]
    pub fn set_span(&mut self, span: Span) {
        self.span = span;
    }
}

impl PartialEq for LitVerbatim {
    fn eq(&self, other: &Self) -> bool {
        self.repr == other.repr
    }
}

impl Eq for LitVerbatim {}

impl std::hash::Hash for LitVerbatim {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.repr.hash(state);
    }
}

impl std::fmt::Display for LitVerbatim {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.repr)
    }
}

impl Spanner for LitVerbatim {
    fn span(&self) -> Span {
        self.span
    }
}

impl From<LitVerbatim> for Lit {
    fn from(value: LitVerbatim) -> Self {
        Lit::Verbatim(value)
    }
}

#[cfg(feature = "serde")]
impl From<LitVerbatim> for String {
    fn from(value: LitVerbatim) -> Self {
        value.repr.into_string()
    }
}
