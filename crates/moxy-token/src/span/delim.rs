use crate::Span;

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct DelimSpan {
    open: Span,
    close: Span,
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
    pub fn new(open: Span, close: Span) -> Self {
        Self { open, close }
    }

    pub fn open(&self) -> Span {
        self.open
    }

    pub fn close(&self) -> Span {
        self.close
    }

    pub fn span(&self) -> Span {
        self.open.join(self.close)
    }
}

impl From<DelimSpan> for Span {
    fn from(value: DelimSpan) -> Self {
        value.span()
    }
}
