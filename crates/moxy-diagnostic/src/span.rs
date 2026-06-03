use moxy_token::Span;

use crate::*;

/// Convenience constructors for building [`Diagnostic`]s from a [`Span`].
pub trait SpanExt {
    fn error(&self, message: impl std::fmt::Display) -> Diagnostic;
    fn warn(&self, message: impl std::fmt::Display) -> Diagnostic;
    fn note(&self, message: impl std::fmt::Display) -> Diagnostic;
    fn help(&self, message: impl std::fmt::Display) -> Diagnostic;
}

impl SpanExt for Span {
    fn error(&self, message: impl std::fmt::Display) -> Diagnostic {
        Diagnostic::new().level(Level::Error).span(*self).message(message).build()
    }

    fn warn(&self, message: impl std::fmt::Display) -> Diagnostic {
        Diagnostic::new().level(Level::Warning).span(*self).message(message).build()
    }

    fn note(&self, message: impl std::fmt::Display) -> Diagnostic {
        Diagnostic::new().level(Level::Note).span(*self).message(message).build()
    }

    fn help(&self, message: impl std::fmt::Display) -> Diagnostic {
        Diagnostic::new().level(Level::Help).span(*self).message(message).build()
    }
}
