use super::UInt;
use crate::lit::Lit;
use crate::parser::{ParseError, ParseStream};
use crate::{LexError, Parse, Span, Spanner};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(into = "String"))]
pub struct LitU8 {
    value: u8,
    suffixed: bool,
    repr: Box<str>,
    span: Span,
}

impl LitU8 {
    #[inline]
    pub fn new(value: u8, suffixed: bool, span: Span) -> Self {
        let repr = if suffixed {
            format!("{value}u8").into_boxed_str()
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
    pub(crate) fn from_parts(value: u8, suffixed: bool, repr: &str, span: Span) -> Self {
        Self {
            value,
            suffixed,
            repr: repr.into(),
            span,
        }
    }

    #[inline]
    pub fn value(&self) -> u8 {
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

impl PartialEq for LitU8 {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.suffixed == other.suffixed
    }
}

impl Eq for LitU8 {}

impl std::hash::Hash for LitU8 {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.repr.hash(state);
    }
}

impl std::fmt::Display for LitU8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.repr)
    }
}

impl Spanner for LitU8 {
    fn span(&self) -> Span {
        self.span
    }
}

impl Parse for LitU8 {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let at = stream.span();

        match Lit::parse(stream)? {
            Lit::UInt(UInt::U8(v)) => Ok(v),
            _ => Err(LexError::new(at).message("expected `u8` literal").into()),
        }
    }
}

impl From<LitU8> for UInt {
    fn from(value: LitU8) -> Self {
        UInt::U8(value)
    }
}

impl From<LitU8> for Lit {
    fn from(value: LitU8) -> Self {
        Lit::UInt(UInt::U8(value))
    }
}

#[cfg(feature = "serde")]
impl From<LitU8> for String {
    fn from(value: LitU8) -> Self {
        value.repr.into_string()
    }
}
