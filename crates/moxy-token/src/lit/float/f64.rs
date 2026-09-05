use super::LitFloat;
use crate::lit::Lit;
use crate::{Span, Spanner};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(into = "String"))]
pub struct LitF64 {
    value: f64,
    suffixed: bool,
    repr: Box<str>,
    span: Span,
}

impl LitF64 {
    #[inline]
    pub fn new(value: f64, suffixed: bool, span: Span) -> Self {
        let repr = if suffixed {
            format!("{value}f64").into_boxed_str()
        } else {
            format!("{value}").into_boxed_str()
        };

        Self {
            value,
            suffixed,
            repr,
            span,
        }
    }

    #[inline]
    pub(crate) fn from_parts(value: f64, suffixed: bool, repr: &str, span: Span) -> Self {
        Self {
            value,
            suffixed,
            repr: repr.into(),
            span,
        }
    }

    #[inline]
    pub fn value(&self) -> f64 {
        self.value
    }

    #[inline]
    pub fn suffixed(&self) -> bool {
        self.suffixed
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

impl PartialEq for LitF64 {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.suffixed == other.suffixed
    }
}

impl Eq for LitF64 {}

impl std::hash::Hash for LitF64 {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.repr.hash(state);
    }
}

impl std::fmt::Display for LitF64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.repr)
    }
}

impl Spanner for LitF64 {
    fn span(&self) -> Span {
        self.span
    }
}

impl From<LitF64> for LitFloat {
    fn from(value: LitF64) -> Self {
        Self::F64(value)
    }
}

impl From<LitF64> for Lit {
    fn from(value: LitF64) -> Self {
        Self::Float(LitFloat::F64(value))
    }
}

#[cfg(feature = "serde")]
impl From<LitF64> for String {
    fn from(value: LitF64) -> Self {
        value.repr.into_string()
    }
}
