use super::LitInt;
use crate::lit::Lit;
use crate::parser::{ParseError, ParseStream};
use crate::{LexError, Parse, Span, Spanner};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(into = "String"))]
pub struct LitI32 {
    value: i32,
    suffixed: bool,
    repr: Box<str>,
    span: Span,
}

impl LitI32 {
    #[inline]
    pub fn new(value: i32, suffixed: bool, span: Span) -> Self {
        let repr = if suffixed {
            format!("{value}i32").into_boxed_str()
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
    pub(crate) fn from_parts(value: i32, suffixed: bool, repr: &str, span: Span) -> Self {
        Self {
            value,
            suffixed,
            repr: repr.into(),
            span,
        }
    }

    #[inline]
    pub fn value(&self) -> i32 {
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

impl PartialEq for LitI32 {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.suffixed == other.suffixed
    }
}

impl Eq for LitI32 {}

impl std::hash::Hash for LitI32 {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.repr.hash(state);
    }
}

impl std::fmt::Display for LitI32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.repr)
    }
}

impl Spanner for LitI32 {
    fn span(&self) -> Span {
        self.span
    }
}

impl Parse for LitI32 {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let at = stream.span();

        match Lit::parse(stream)? {
            Lit::Int(LitInt::I32(v)) => Ok(v),
            _ => Err(LexError::new(at).message("expected `i32` literal").into()),
        }
    }
}

impl From<LitI32> for LitInt {
    fn from(value: LitI32) -> Self {
        Self::I32(value)
    }
}

impl From<LitI32> for Lit {
    fn from(value: LitI32) -> Self {
        Self::Int(LitInt::I32(value))
    }
}

#[cfg(feature = "serde")]
impl From<LitI32> for String {
    fn from(value: LitI32) -> Self {
        value.repr.into_string()
    }
}
