use super::LitInt;
use crate::lit::Lit;
use crate::parser::{ParseError, ParseStream};
use crate::{LexError, Parse, Span, Spanner};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(into = "String"))]
pub struct LitI128 {
    value: i128,
    suffixed: bool,
    repr: Box<str>,
    span: Span,
}

impl LitI128 {
    #[inline]
    pub fn new(value: i128, suffixed: bool, span: Span) -> Self {
        let repr = if suffixed {
            format!("{value}i128").into_boxed_str()
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
    pub(crate) fn from_parts(value: i128, suffixed: bool, repr: &str, span: Span) -> Self {
        Self {
            value,
            suffixed,
            repr: repr.into(),
            span,
        }
    }

    #[inline]
    pub fn value(&self) -> i128 {
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

impl PartialEq for LitI128 {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.suffixed == other.suffixed
    }
}

impl Eq for LitI128 {}

impl std::hash::Hash for LitI128 {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.repr.hash(state);
    }
}

impl std::fmt::Display for LitI128 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.repr)
    }
}

impl Spanner for LitI128 {
    fn span(&self) -> Span {
        self.span
    }
}

impl Parse for LitI128 {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let at = stream.span();

        match Lit::parse(stream)? {
            Lit::Int(LitInt::I128(v)) => Ok(v),
            _ => Err(LexError::new(at).message("expected `i128` literal").into()),
        }
    }
}

impl From<LitI128> for LitInt {
    fn from(value: LitI128) -> Self {
        Self::I128(value)
    }
}

impl From<LitI128> for Lit {
    fn from(value: LitI128) -> Self {
        Self::Int(LitInt::I128(value))
    }
}

#[cfg(feature = "serde")]
impl From<LitI128> for String {
    fn from(value: LitI128) -> Self {
        value.repr.into_string()
    }
}
