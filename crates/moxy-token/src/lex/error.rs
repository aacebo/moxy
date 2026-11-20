use crate::Span;

/// An error returned while lexing Rust source into tokens.
#[derive(Debug, Clone)]
pub struct LexError {
    span: Span,
    message: Option<String>,
}

impl LexError {
    pub fn new(span: Span) -> Self {
        Self { span, message: None }
    }

    pub fn message(mut self, message: impl std::fmt::Display) -> Self {
        self.message = Some(message.to_string());
        self
    }

    pub fn span(&self) -> Span {
        self.span
    }

    pub fn is_reject(&self) -> bool {
        self.message.is_none()
    }

    // Groups nested past `MAX_DEPTH`, which lexing and the conversions from
    // `proc_macro` and `proc_macro2` refuse. The parser reports its own depth
    // error with another message.
    pub(crate) fn too_deep(span: Span) -> Self {
        Self::new(span).message(format!("delimiters nest deeper than {} levels", super::MAX_DEPTH))
    }
}

impl<T> From<LexError> for Result<T, LexError> {
    fn from(value: LexError) -> Self {
        Self::Err(value)
    }
}

impl std::fmt::Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message.as_deref().unwrap_or("invalid token"))
    }
}

impl std::error::Error for LexError {}
