use super::LitInt;
use crate::lit::Lit;
use crate::parser::{ParseError, ParseStream};
use crate::{LexError, Parse, Span, Spanner};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(into = "String"))]
pub struct LitISize {
    value: isize,
    suffixed: bool,
    repr: Box<str>,
    span: Span,
}

impl LitISize {
    #[inline]
    pub fn new(value: isize, suffixed: bool, span: Span) -> Self {
        let repr = if suffixed {
            format!("{value}isize").into_boxed_str()
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
    pub(crate) fn from_parts(value: isize, suffixed: bool, repr: &str, span: Span) -> Self {
        Self {
            value,
            suffixed,
            repr: repr.into(),
            span,
        }
    }

    #[inline]
    pub fn value(&self) -> isize {
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

impl PartialEq for LitISize {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.suffixed == other.suffixed
    }
}

impl Eq for LitISize {}

impl std::hash::Hash for LitISize {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.repr.hash(state);
    }
}

impl std::fmt::Display for LitISize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.repr)
    }
}

impl Spanner for LitISize {
    fn span(&self) -> Span {
        self.span
    }
}

impl Parse for LitISize {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let at = stream.span();

        match Lit::parse(stream)? {
            Lit::Int(LitInt::ISize(v)) => Ok(v),
            _ => Err(LexError::new(at).message("expected `isize` literal").into()),
        }
    }
}

impl From<LitISize> for LitInt {
    fn from(value: LitISize) -> Self {
        Self::ISize(value)
    }
}

impl From<LitISize> for Lit {
    fn from(value: LitISize) -> Self {
        Self::Int(LitInt::ISize(value))
    }
}

#[cfg(feature = "serde")]
impl From<LitISize> for String {
    fn from(value: LitISize) -> Self {
        value.repr.into_string()
    }
}
