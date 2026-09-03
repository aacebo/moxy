#![cfg_attr(nightly, feature(proc_macro_diagnostic))]

use moxy_token::Token;

extern crate proc_macro;

mod level;
mod span;

#[doc(inline)]
pub use level::*;

use moxy_token::span::DelimSpan;
#[doc(inline)]
pub use span::*;

use moxy_ast::ParseError;
use moxy_token::{Delim, Group, Ident, Lit, Punctuation, Span, ToTokenStream, ToTokens, TokenStream};

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
            return Default::default();
        }

        if !self.level.is_error() {
            return Default::default();
        }

        self.to_compile_error()
    }

    pub fn to_compile_error(&self) -> TokenStream {
        let mut tokens = TokenStream::new();
        let start = self.spans.first().copied().unwrap_or_default();
        let end = self.spans.last().copied().unwrap_or_default();
        let span = start.join(end);

        fn cerror(span: Span, message: impl std::fmt::Display) -> TokenStream {
            let ident = Ident::new("compile_error").with_span(span);
            let bang = <Token![!]>::new(span);
            let mut lit = Lit::string(&message.to_string());
            lit.set_span(span);

            let mut group = Group::new(Delim::Paren, lit.into_token_tree().into_token_stream());
            group.set_span(DelimSpan::new(span, span));

            vec![
                ident.into_token_tree(),
                Punctuation::from(bang).into_token_tree(),
                group.into_token_tree(),
                Punctuation::from(<Token![;]>::new(span)).into_token_tree(),
            ]
            .into_token_stream()
        }

        if let Some(msg) = &self.message {
            tokens.extend(cerror(span, msg));
        }

        for child in &self.children {
            tokens.extend(child.to_compile_error());
        }

        tokens
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
        if let Some(msg) = &self.message {
            write!(f, "{}", msg)?;
        }

        for child in &self.children {
            write!(f, "\n  {}", child)?;
        }

        Ok(())
    }
}

impl From<&ParseError> for Diagnostic {
    fn from(err: &ParseError) -> Self {
        let mut builder = Self::new().level(Level::Error).span(err.span()).message(err.message());

        for child in err.children() {
            builder = builder.child(Self::from(child));
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
