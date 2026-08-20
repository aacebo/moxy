use super::Int;
use crate::lit::Lit;
use crate::parser::{ParseError, ParseStream};
use crate::{LexError, Parse, Span, Spanner};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(into = "String"))]
pub struct LitI8 {
    value: i8,
    suffixed: bool,
    repr: Box<str>,
    span: Span,
}

impl LitI8 {
    #[inline]
    pub fn new(value: i8, suffixed: bool, span: Span) -> Self {
        let repr = if suffixed {
            format!("{value}i8").into_boxed_str()
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
    pub(crate) fn from_parts(value: i8, suffixed: bool, repr: &str, span: Span) -> Self {
        Self {
            value,
            suffixed,
            repr: repr.into(),
            span,
        }
    }

    #[inline]
    pub fn value(&self) -> i8 {
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

impl PartialEq for LitI8 {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.suffixed == other.suffixed
    }
}

impl Eq for LitI8 {}

impl std::hash::Hash for LitI8 {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.repr.hash(state);
    }
}

impl std::fmt::Display for LitI8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.repr)
    }
}

impl Spanner for LitI8 {
    fn span(&self) -> Span {
        self.span
    }
}

impl Parse for LitI8 {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let at = stream.span();

        match Lit::parse(stream)? {
            Lit::Int(Int::I8(v)) => Ok(v),
            _ => Err(LexError::new(at).message("expected `i8` literal").into()),
        }
    }
}

impl From<LitI8> for Int {
    fn from(value: LitI8) -> Self {
        Int::I8(value)
    }
}

impl From<LitI8> for Lit {
    fn from(value: LitI8) -> Self {
        Lit::Int(Int::I8(value))
    }
}

#[cfg(feature = "serde")]
impl From<LitI8> for String {
    fn from(value: LitI8) -> Self {
        value.repr.into_string()
    }
}
