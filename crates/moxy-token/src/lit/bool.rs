use crate::lit::Lit;
use crate::parser::{ParseError, ParseStream};
use crate::{LexError, Parse, Span, Spanner};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(into = "String"))]
pub struct LitBool {
    value: bool,
    span: Span,
}

impl LitBool {
    #[inline]
    pub fn new(value: bool, span: Span) -> Self {
        Self { value, span }
    }

    #[inline]
    pub fn value(&self) -> bool {
        self.value
    }

    #[inline]
    pub fn repr(&self) -> &str {
        if self.value { "true" } else { "false" }
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

impl PartialEq for LitBool {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for LitBool {}

impl std::hash::Hash for LitBool {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl std::fmt::Display for LitBool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.repr())
    }
}

impl Spanner for LitBool {
    fn span(&self) -> Span {
        self.span
    }
}

impl Parse for LitBool {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let at = stream.span();

        match stream.parse::<Lit>()? {
            Lit::Bool(v) => Ok(v),
            _ => Err(LexError::new(at).message("expected bool literal").into()),
        }
    }
}

impl From<LitBool> for Lit {
    fn from(value: LitBool) -> Self {
        Self::Bool(value)
    }
}

#[cfg(feature = "serde")]
impl From<LitBool> for String {
    fn from(value: LitBool) -> Self {
        value.repr().to_string()
    }
}
