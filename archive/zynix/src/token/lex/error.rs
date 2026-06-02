use crate::Span;

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
}

impl<T> From<LexError> for Result<T, LexError> {
    fn from(value: LexError) -> Self {
        Self::Err(value)
    }
}

impl std::fmt::Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message.as_deref().unwrap_or("string could not be parsed"))
    }
}

impl std::error::Error for LexError {}
