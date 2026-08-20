use super::Int;
use crate::lit::Lit;
use crate::parser::{ParseError, ParseStream};
use crate::{LexError, Parse, Span, Spanner};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(into = "String"))]
pub struct LitIsize {
    value: isize,
    suffixed: bool,
    repr: Box<str>,
    span: Span,
}

impl LitIsize {
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

impl PartialEq for LitIsize {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.suffixed == other.suffixed
    }
}

impl Eq for LitIsize {}

impl std::hash::Hash for LitIsize {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.repr.hash(state);
    }
}

impl std::fmt::Display for LitIsize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.repr)
    }
}

impl Spanner for LitIsize {
    fn span(&self) -> Span {
        self.span
    }
}

impl Parse for LitIsize {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let at = stream.span();

        match Lit::parse(stream)? {
            Lit::Int(Int::Isize(v)) => Ok(v),
            _ => Err(LexError::new(at).message("expected `isize` literal").into()),
        }
    }
}

impl From<LitIsize> for Int {
    fn from(value: LitIsize) -> Self {
        Int::Isize(value)
    }
}

impl From<LitIsize> for Lit {
    fn from(value: LitIsize) -> Self {
        Lit::Int(Int::Isize(value))
    }
}

#[cfg(feature = "serde")]
impl From<LitIsize> for String {
    fn from(value: LitIsize) -> Self {
        value.repr.into_string()
    }
}
