use super::LitUInt;
use crate::lit::Lit;
use crate::parser::{ParseError, ParseStream};
use crate::{LexError, Parse, Span, Spanner};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(into = "String"))]
pub struct LitU128 {
    value: u128,
    suffixed: bool,
    repr: Box<str>,
    span: Span,
}

impl LitU128 {
    #[inline]
    pub fn new(value: u128, suffixed: bool, span: Span) -> Self {
        let repr = if suffixed {
            format!("{value}u128").into_boxed_str()
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
    pub(crate) fn from_parts(value: u128, suffixed: bool, repr: &str, span: Span) -> Self {
        Self {
            value,
            suffixed,
            repr: repr.into(),
            span,
        }
    }

    #[inline]
    pub fn value(&self) -> u128 {
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

impl PartialEq for LitU128 {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.suffixed == other.suffixed
    }
}

impl Eq for LitU128 {}

impl std::hash::Hash for LitU128 {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.repr.hash(state);
    }
}

impl std::fmt::Display for LitU128 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.repr)
    }
}

impl Spanner for LitU128 {
    fn span(&self) -> Span {
        self.span
    }
}

impl Parse for LitU128 {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let at = stream.span();

        match Lit::parse(stream)? {
            Lit::UInt(LitUInt::U128(v)) => Ok(v),
            _ => Err(LexError::new(at).message("expected `u128` literal").into()),
        }
    }
}

impl From<LitU128> for LitUInt {
    fn from(value: LitU128) -> Self {
        Self::U128(value)
    }
}

impl From<LitU128> for Lit {
    fn from(value: LitU128) -> Self {
        Self::UInt(LitUInt::U128(value))
    }
}

#[cfg(feature = "serde")]
impl From<LitU128> for String {
    fn from(value: LitU128) -> Self {
        value.repr.into_string()
    }
}
