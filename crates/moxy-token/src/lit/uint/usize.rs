use super::LitUInt;
use crate::lit::Lit;
use crate::parser::{ParseError, ParseStream};
use crate::{LexError, Parse, Span, Spanner};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(into = "String"))]
pub struct LitUSize {
    value: usize,
    suffixed: bool,
    repr: Box<str>,
    span: Span,
}

impl LitUSize {
    #[inline]
    pub fn new(value: usize, suffixed: bool, span: Span) -> Self {
        let repr = if suffixed {
            format!("{value}usize").into_boxed_str()
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
    pub(crate) fn from_parts(value: usize, suffixed: bool, repr: &str, span: Span) -> Self {
        Self {
            value,
            suffixed,
            repr: repr.into(),
            span,
        }
    }

    #[inline]
    pub fn value(&self) -> usize {
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

impl PartialEq for LitUSize {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.suffixed == other.suffixed
    }
}

impl Eq for LitUSize {}

impl std::hash::Hash for LitUSize {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.repr.hash(state);
    }
}

impl std::fmt::Display for LitUSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.repr)
    }
}

impl Spanner for LitUSize {
    fn span(&self) -> Span {
        self.span
    }
}

impl Parse for LitUSize {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let at = stream.span();

        match Lit::parse(stream)? {
            Lit::UInt(LitUInt::USize(v)) => Ok(v),
            _ => Err(LexError::new(at).message("expected `usize` literal").into()),
        }
    }
}

impl From<LitUSize> for LitUInt {
    fn from(value: LitUSize) -> Self {
        Self::USize(value)
    }
}

impl From<LitUSize> for Lit {
    fn from(value: LitUSize) -> Self {
        Self::UInt(LitUInt::USize(value))
    }
}

#[cfg(feature = "serde")]
impl From<LitUSize> for String {
    fn from(value: LitUSize) -> Self {
        value.repr.into_string()
    }
}
