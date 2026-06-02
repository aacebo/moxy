use crate::report::Level;
use crate::token::{ToTokenStream, ToTokens};
use crate::{Span, TokenStream};

#[derive(Debug, Clone)]
pub struct Diagnostic {
    level: Level,
    spans: Vec<Span>,
    message: Option<String>,
    children: Vec<Self>,
}

impl Diagnostic {
    pub fn new() -> build::Builder {
        build::Builder::new()
    }

    /// the max level of this diagnostic and its children.
    pub fn level(&self) -> Level {
        self.level
    }

    pub fn spans(&self) -> &[Span] {
        &self.spans
    }

    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }

    pub fn children(&self) -> &[Self] {
        &self.children
    }

    pub fn emit(self) -> TokenStream {
        #[cfg(nightly)]
        if proc_macro::is_available() {
            proc_macro::Diagnostic::from(self.clone()).emit();
        }

        self.into_token_stream()
    }
}

#[cfg(nightly)]
impl From<Diagnostic> for proc_macro::Diagnostic {
    fn from(value: Diagnostic) -> Self {
        let msg = value.message.unwrap_or_default();
        let spans: Vec<_> = value.spans.into_iter().map(proc_macro::Span::from).collect();

        let mut new = if spans.is_empty() {
            Self::new(value.level.into(), msg)
        } else {
            Self::spanned(spans, value.level.into(), msg)
        };

        for child in value.children {
            let message = child.message.unwrap_or_default();
            let spans: Vec<_> = child.spans.into_iter().map(proc_macro::Span::from).collect();

            if child.level.is_error() {
                new = new.span_error(spans, message);
            } else if child.level.is_help() {
                new = new.span_help(spans, message);
            } else if child.level.is_note() {
                new = new.span_note(spans, message);
            } else if child.level.is_warning() {
                new = new.span_warning(spans, message);
            }
        }

        new
    }
}

impl std::fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]:", self.level)?;

        if let Some(msg) = &self.message {
            write!(f, ": {}", msg)?;
        }

        for child in &self.children {
            write!(f, "\n  {}", child)?;
        }

        Ok(())
    }
}

impl Eq for Diagnostic {}

impl PartialEq for Diagnostic {
    fn eq(&self, other: &Self) -> bool {
        self.spans == other.spans
    }
}

impl ToTokens for Diagnostic {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.clone().emit().to_tokens(tokens);
    }
}

#[doc(hidden)]
pub mod build {
    use super::*;

    #[doc(hidden)]
    #[derive(Debug, Clone)]
    pub struct Builder {
        level: Level,
        spans: Vec<Span>,
        message: Option<String>,
        children: Vec<Diagnostic>,
    }

    impl Default for Builder {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Builder {
        pub fn new() -> Self {
            Self {
                level: Level::Unknown,
                spans: vec![],
                message: None,
                children: vec![],
            }
        }

        pub fn span(mut self, span: Span) -> Self {
            self.spans.push(span);
            self
        }

        pub fn spans(mut self, spans: impl Iterator<Item = Span>) -> Self {
            self.spans.extend(spans);
            self
        }

        pub fn level(mut self, level: Level) -> Self {
            self.level = level;
            self
        }

        pub fn message(mut self, message: impl std::fmt::Display) -> Self {
            self.message = Some(message.to_string());
            self
        }

        pub fn child(mut self, child: Diagnostic) -> Self {
            self.children.push(child);
            self
        }

        pub fn build(self) -> Diagnostic {
            let mut level = self.level;

            for child in &self.children {
                let clevel = child.level();

                if clevel > level {
                    level = clevel;
                }
            }

            Diagnostic {
                spans: self.spans,
                level,
                message: self.message,
                children: self.children,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Span;

    #[test]
    fn diagnostic_new_sets_level() {
        let d = Diagnostic::new().level(Level::Warning).build();
        assert_eq!(d.level(), Level::Warning);
    }

    #[test]
    fn level_elevated_by_child() {
        let child = Diagnostic::new().level(Level::Error).message("child").build();
        let parent = Diagnostic::new().level(Level::Note).message("parent").child(child).build();

        assert_eq!(parent.level(), Level::Error);
    }

    #[test]
    fn level_not_lowered_by_child() {
        let child = Diagnostic::new().level(Level::Note).message("child").build();
        let parent = Diagnostic::new().level(Level::Error).message("parent").child(child).build();

        assert_eq!(parent.level(), Level::Error);
    }

    #[test]
    fn level_max_across_multiple_children() {
        let c1 = Diagnostic::new().level(Level::Note).build();
        let c2 = Diagnostic::new().level(Level::Warning).build();
        let c3 = Diagnostic::new().level(Level::Help).build();
        let parent = Diagnostic::new().level(Level::Unknown).child(c1).child(c2).child(c3).build();

        assert_eq!(parent.level(), Level::Warning);
    }

    #[test]
    fn multiple_spans() {
        let s1 = Span::default();
        let s2 = Span::default();
        let d = Diagnostic::new().spans(vec![s1, s2].into_iter()).build();
        assert_eq!(d.spans().len(), 2);
    }

    #[test]
    fn display_with_message() {
        let d = Diagnostic::new().level(Level::Error).message("something broke").build();
        let s = format!("{}", d);
        assert_eq!(s, "[error]:: something broke");
    }

    #[test]
    fn display_without_message() {
        let d = Diagnostic::new().level(Level::Warning).build();
        let s = format!("{}", d);
        assert_eq!(s, "[warning]:");
    }

    #[test]
    fn display_with_children() {
        let child = Diagnostic::new().level(Level::Help).message("try this").build();
        let parent = Diagnostic::new().level(Level::Error).message("failed").child(child).build();
        let s = format!("{}", parent);
        assert!(s.contains("[error]:: failed"));
        assert!(s.contains("\n  [help]:: try this"));
    }

    #[test]
    fn partial_eq_same_spans() {
        let span = Span::default();
        let d1 = Diagnostic::new().level(Level::Error).message("a").span(span).build();
        let d2 = Diagnostic::new().level(Level::Note).message("b").span(span).build();
        assert_eq!(d1, d2);
    }

    #[test]
    fn partial_eq_no_spans() {
        let d1 = Diagnostic::new().level(Level::Error).message("a").build();
        let d2 = Diagnostic::new().level(Level::Note).message("b").build();
        // Both have empty spans, so they are equal
        assert_eq!(d1, d2);
    }

    #[cfg(not(nightly))]
    #[test]
    fn to_stream_produces_compile_error() {
        let d = Diagnostic::new().level(Level::Error).message("broken").build();
        let stream = d.to_token_stream();
        let s = stream.to_string();
        assert!(s.contains("compile_error"), "expected compile_error in: {}", s);
        assert!(s.contains("broken"), "expected message in: {}", s);
    }

    #[cfg(not(nightly))]
    #[test]
    fn emit_returns_stream() {
        let d = Diagnostic::new().level(Level::Warning).message("warn msg").build();
        let stream = d.emit();
        let s = stream.to_string();
        assert!(s.contains("compile_error"), "expected compile_error in: {}", s);
        assert!(s.contains("warn msg"), "expected message in: {}", s);
    }

    #[cfg(not(nightly))]
    #[test]
    fn to_stream_includes_children() {
        let child = Diagnostic::new().level(Level::Help).message("hint").build();
        let parent = Diagnostic::new()
            .level(Level::Error)
            .message("main error")
            .child(child)
            .build();
        let s = parent.to_token_stream().to_string();
        assert!(s.contains("compile_error"));
        assert!(s.contains("main error"));
        assert!(s.contains("hint"));
    }

    #[cfg(not(nightly))]
    #[test]
    fn to_stream_no_message() {
        let d = Diagnostic::new().level(Level::Error).build();
        let s = d.to_token_stream().to_string();
        assert!(s.contains("compile_error"));
    }

    #[test]
    fn span_error_helper() {
        let span = Span::default();
        let d = span.error("err msg");
        assert_eq!(d.level(), Level::Error);
        assert_eq!(d.message(), Some("err msg"));
        assert_eq!(d.spans().len(), 1);
        assert_eq!(d.spans()[0], span);
    }

    #[test]
    fn span_warn_helper() {
        let span = Span::default();
        let d = span.warn("warn msg");
        assert_eq!(d.level(), Level::Warning);
        assert_eq!(d.message(), Some("warn msg"));
    }

    #[test]
    fn span_note_helper() {
        let span = Span::default();
        let d = span.note("note msg");
        assert_eq!(d.level(), Level::Note);
        assert_eq!(d.message(), Some("note msg"));
    }

    #[test]
    fn span_help_helper() {
        let span = Span::default();
        let d = span.help("help msg");
        assert_eq!(d.level(), Level::Help);
        assert_eq!(d.message(), Some("help msg"));
    }
}
