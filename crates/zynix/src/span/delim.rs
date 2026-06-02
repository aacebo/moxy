use crate::Span;

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct DelimSpan {
    open: Span,
    close: Span,
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
