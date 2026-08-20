#![cfg_attr(nightly, feature(proc_macro_diagnostic))]

extern crate proc_macro;

mod level;
mod span;

#[doc(inline)]
pub use level::*;
use moxy_token::parser::ParseError;
use moxy_token::punct::Not;
use moxy_token::{Delim, Group, Ident, Lit, Punctuation, Span, ToTokenStream, ToTokens, TokenStream, TokenTree};
#[doc(inline)]
pub use span::*;

/// Build a note-level [`Diagnostic`].
///
/// Returns a [`Diagnostic`] (not yet emitted) — call `.emit()` to turn it into a
/// `TokenStream`, or nest it inside another diagnostic's `[ … ]` children list.
///
/// # Forms
///
/// ```ignore
/// note!("message");
/// note!("message", span = some_span);
/// note!("message", [ help!("hint") ]);
/// note!("message", span = some_span, [ help!("hint") ]);
/// ```
#[macro_export]
macro_rules! note {
    ($message:expr) => {
        $crate::Diagnostic::new()
            .level($crate::Level::Note)
            .message($message)
            .build()
    };
    ($message:expr, span = $span:expr) => {
        $crate::Diagnostic::new()
            .level($crate::Level::Note)
            .span($span)
            .message($message)
            .build()
    };
    ($message:expr, [ $($child:expr),* $(,)? ]) => {{
        let mut __diag = $crate::Diagnostic::new()
            .level($crate::Level::Note)
            .message($message);
        $( __diag = __diag.child($child); )*
        __diag.build()
    }};
    ($message:expr, span = $span:expr, [ $($child:expr),* $(,)? ]) => {{
        let mut __diag = $crate::Diagnostic::new()
            .level($crate::Level::Note)
            .span($span)
            .message($message);
        $( __diag = __diag.child($child); )*
        __diag.build()
    }};
}

/// Build a warning-level [`Diagnostic`].
///
/// Returns a [`Diagnostic`] (not yet emitted) — call `.emit()` to turn it into a
/// `TokenStream`, or nest it inside another diagnostic's `[ … ]` children list.
///
/// # Forms
///
/// ```ignore
/// warn!("message");
/// warn!("message", span = some_span);
/// warn!("message", [ help!("hint") ]);
/// warn!("message", span = some_span, [ help!("hint") ]);
/// ```
#[macro_export]
macro_rules! warn {
    ($message:expr) => {
        $crate::Diagnostic::new()
            .level($crate::Level::Warning)
            .message($message)
            .build()
    };
    ($message:expr, span = $span:expr) => {
        $crate::Diagnostic::new()
            .level($crate::Level::Warning)
            .span($span)
            .message($message)
            .build()
    };
    ($message:expr, [ $($child:expr),* $(,)? ]) => {{
        let mut __diag = $crate::Diagnostic::new()
            .level($crate::Level::Warning)
            .message($message);
        $( __diag = __diag.child($child); )*
        __diag.build()
    }};
    ($message:expr, span = $span:expr, [ $($child:expr),* $(,)? ]) => {{
        let mut __diag = $crate::Diagnostic::new()
            .level($crate::Level::Warning)
            .span($span)
            .message($message);
        $( __diag = __diag.child($child); )*
        __diag.build()
    }};
}

/// Build an error-level [`Diagnostic`].
///
/// Returns a [`Diagnostic`] (not yet emitted) — call `.emit()` to turn it into a
/// `TokenStream`, or nest it inside another diagnostic's `[ … ]` children list.
///
/// # Forms
///
/// ```ignore
/// error!("message");
/// error!("message", span = some_span);
/// error!("message", [ help!("hint") ]);
/// error!("message", span = some_span, [ help!("hint") ]);
/// ```
#[macro_export]
macro_rules! error {
    ($message:expr) => {
        $crate::Diagnostic::new()
            .level($crate::Level::Error)
            .message($message)
            .build()
    };
    ($message:expr, span = $span:expr) => {
        $crate::Diagnostic::new()
            .level($crate::Level::Error)
            .span($span)
            .message($message)
            .build()
    };
    ($message:expr, [ $($child:expr),* $(,)? ]) => {{
        let mut __diag = $crate::Diagnostic::new()
            .level($crate::Level::Error)
            .message($message);
        $( __diag = __diag.child($child); )*
        __diag.build()
    }};
    ($message:expr, span = $span:expr, [ $($child:expr),* $(,)? ]) => {{
        let mut __diag = $crate::Diagnostic::new()
            .level($crate::Level::Error)
            .span($span)
            .message($message);
        $( __diag = __diag.child($child); )*
        __diag.build()
    }};
}

/// Build a help-level [`Diagnostic`], typically nested as a child of another
/// diagnostic.
///
/// Like the other macros it returns a [`Diagnostic`]; it carries supplementary
/// guidance and is usually placed in a parent's `[ … ]` children list.
///
/// # Forms
///
/// ```ignore
/// help!("try this instead");
/// help!("try this instead", span = some_span);
/// help!("try this instead", [ help!("and also this") ]);
/// help!("try this instead", span = some_span, [ help!("and also this") ]);
/// ```
#[macro_export]
macro_rules! help {
    ($message:expr) => {
        $crate::Diagnostic::new()
            .level($crate::Level::Help)
            .message($message)
            .build()
    };
    ($message:expr, span = $span:expr) => {
        $crate::Diagnostic::new()
            .level($crate::Level::Help)
            .span($span)
            .message($message)
            .build()
    };
    ($message:expr, [ $($child:expr),* $(,)? ]) => {{
        let mut __diag = $crate::Diagnostic::new()
            .level($crate::Level::Help)
            .message($message);
        $( __diag = __diag.child($child); )*
        __diag.build()
    }};
    ($message:expr, span = $span:expr, [ $($child:expr),* $(,)? ]) => {{
        let mut __diag = $crate::Diagnostic::new()
            .level($crate::Level::Help)
            .span($span)
            .message($message);
        $( __diag = __diag.child($child); )*
        __diag.build()
    }};
}

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

    pub fn child(mut self, child: Self) -> Self {
        self.children.push(child);
        self
    }

    pub fn emit(self) -> TokenStream {
        #[cfg(nightly)]
        if proc_macro::is_available() {
            proc_macro::Diagnostic::from(self.clone()).emit();
        }

        self.to_compile_error()
    }

    pub fn to_compile_error(&self) -> TokenStream {
        let span = self.spans.first().copied().unwrap_or_default();
        let ident = Ident::new("compile_error").with_span(span);
        let bang = Not::new(span);
        let mut lit = Lit::string(&self.to_string());
        lit.set_span(span);

        let inner: TokenTree = lit.into();
        let group = Group::new(Delim::Paren, inner.into_token_stream());

        vec![
            TokenTree::from(ident),
            TokenTree::from(Punctuation::from(bang)),
            TokenTree::from(group),
        ]
        .into()
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

impl From<&ParseError> for Diagnostic {
    fn from(err: &ParseError) -> Self {
        let mut builder = Diagnostic::new().level(Level::Error).span(err.span()).message(err.message());

        for child in err.children() {
            builder = builder.child(Diagnostic::from(child));
        }

        builder.build()
    }
}

impl From<ParseError> for Diagnostic {
    fn from(err: ParseError) -> Self {
        Self::from(&err)
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
        self.to_compile_error().to_tokens(tokens);
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
    use moxy_token::Span;
    use moxy_token::parser::ParseError;

    use super::*;
    use crate::SpanExt;

    #[test]
    fn diagnostic_new_sets_level() {
        let d = Diagnostic::new().level(Level::Warning).build();
        assert_eq!(d.level(), Level::Warning);
    }

    #[test]
    fn from_parse_error_carries_span_and_message() {
        let err = ParseError::new(Span::default(), "bad input");
        let diag = Diagnostic::from(&err);

        assert_eq!(diag.level(), Level::Error);
        assert_eq!(diag.message(), Some("bad input"));
        assert_eq!(diag.spans().len(), 1);
    }

    #[test]
    fn from_parse_error_attaches_children() {
        let child = ParseError::new(Span::default(), "expected `if`");
        let err = ParseError::new(Span::default(), "bad keyword").combine(child);
        let diag = Diagnostic::from(&err);

        assert_eq!(diag.children().len(), 1);
        assert_eq!(diag.children()[0].message(), Some("expected `if`"));
    }

    #[cfg(not(nightly))]
    #[test]
    fn from_parse_error_emits_compile_error() {
        let err = ParseError::new(Span::default(), "broken input");
        let s = Diagnostic::from(err).emit().to_string();

        assert!(s.contains("compile_error"), "expected compile_error in: {s}");
        assert!(s.contains("broken input"), "expected message in: {s}");
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

    #[test]
    fn help_macro_builds_diagnostic() {
        let d: Diagnostic = crate::help!("do this");
        assert_eq!(d.level(), Level::Help);
        assert_eq!(d.message(), Some("do this"));
    }

    #[test]
    fn help_macro_with_span() {
        let d: Diagnostic = crate::help!("do this", span = Span::default());
        assert_eq!(d.level(), Level::Help);
        assert_eq!(d.spans().len(), 1);
    }

    #[test]
    fn warn_macro_attaches_help_child() {
        let span = Span::default();
        let s = crate::warn!("testi", [crate::help!("do this...", span = span)]).to_string();
        assert!(s.contains("testi"), "expected parent message in: {s}");
        assert!(s.contains("do this..."), "expected child message in: {s}");
    }

    #[test]
    fn error_macro_span_and_children() {
        let s = crate::error!("e", span = Span::default(), [crate::help!("h")]).to_string();
        assert!(s.contains("e"), "expected parent message in: {s}");
        assert!(s.contains("h"), "expected child message in: {s}");
    }

    #[test]
    fn nested_children() {
        let s = crate::warn!("a", [crate::help!("b", [crate::help!("c")])]).to_string();
        assert!(s.contains("a"));
        assert!(s.contains("b"));
        assert!(s.contains("c"));
    }
}
