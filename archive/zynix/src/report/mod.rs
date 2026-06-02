mod diagnostic;
mod level;

pub use diagnostic::*;
pub use level::*;

impl crate::Span {
    pub fn error(&self, message: impl std::fmt::Display) -> Diagnostic {
        Diagnostic::new().level(Level::Error).span(*self).message(message).build()
    }

    pub fn warn(&self, message: impl std::fmt::Display) -> Diagnostic {
        Diagnostic::new().level(Level::Warning).span(*self).message(message).build()
    }

    pub fn note(&self, message: impl std::fmt::Display) -> Diagnostic {
        Diagnostic::new().level(Level::Note).span(*self).message(message).build()
    }

    pub fn help(&self, message: impl std::fmt::Display) -> Diagnostic {
        Diagnostic::new().level(Level::Help).span(*self).message(message).build()
    }
}
