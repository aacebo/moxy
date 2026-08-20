use super::UInt;
use crate::lit::Lit;
use crate::parser::{ParseError, ParseStream};
use crate::{LexError, Parse, Span, Spanner};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(into = "String"))]
pub struct LitUsize {
    value: usize,
    suffixed: bool,
    repr: Box<str>,
    span: Span,
}

impl LitUsize {
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

impl PartialEq for LitUsize {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.suffixed == other.suffixed
    }
}

impl Eq for LitUsize {}

impl std::hash::Hash for LitUsize {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.repr.hash(state);
    }
}

impl std::fmt::Display for LitUsize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.repr)
    }
}

impl Spanner for LitUsize {
    fn span(&self) -> Span {
        self.span
    }
}

impl Parse for LitUsize {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let at = stream.span();

        match Lit::parse(stream)? {
            Lit::UInt(UInt::Usize(v)) => Ok(v),
            _ => Err(LexError::new(at).message("expected `usize` literal").into()),
        }
    }
}

impl From<LitUsize> for UInt {
    fn from(value: LitUsize) -> Self {
        UInt::Usize(value)
    }
}

impl From<LitUsize> for Lit {
    fn from(value: LitUsize) -> Self {
        Lit::UInt(UInt::Usize(value))
    }
}

#[cfg(feature = "serde")]
impl From<LitUsize> for String {
    fn from(value: LitUsize) -> Self {
        value.repr.into_string()
    }
}
