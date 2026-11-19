use crate::Span;

/// The opening and closing spans of a delimited token group, and the span of
/// the whole group.
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct DelimSpan {
    open: Span,
    close: Span,
    whole: Span,
}

impl Default for DelimSpan {
    fn default() -> Self {
        Self::new(Span::call_site(), Span::call_site())
    }
}

// Equality ignores the spans so AST nodes embedding a `DelimSpan` (directly or
// via a delimiter token) compare structurally.
impl PartialEq for DelimSpan {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl Eq for DelimSpan {}

impl std::hash::Hash for DelimSpan {
    fn hash<H: std::hash::Hasher>(&self, _: &mut H) {}
}

impl DelimSpan {
    /// The whole group's span joins `open` and `close`, which on stable keeps
    /// `close` alone for compiler spans; use [`DelimSpan::with_whole`] when the
    /// group's full span is known.
    #[inline]
    pub fn new(open: Span, close: Span) -> Self {
        Self::with_whole(open, close, open.join(close))
    }

    /// Delimiter spans with the whole group's span, as the compiler reports it
    /// for a group it lexed.
    #[inline]
    pub fn with_whole(open: Span, close: Span, whole: Span) -> Self {
        Self { open, close, whole }
    }

    #[inline]
    pub fn open(&self) -> Span {
        self.open
    }

    #[inline]
    pub fn close(&self) -> Span {
        self.close
    }

    #[inline]
    pub fn span(&self) -> Span {
        self.whole
    }
}

impl From<DelimSpan> for Span {
    #[inline]
    fn from(value: DelimSpan) -> Self {
        value.span()
    }
}
