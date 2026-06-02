use crate::Span;

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct RangeSpan {
    start: Span,
    end: Span,
}

impl RangeSpan {
    pub fn new(start: Span, end: Span) -> Self {
        Self { start, end }
    }

    pub fn start(&self) -> Span {
        self.start
    }

    pub fn end(&self) -> Span {
        self.end
    }

    pub fn span(&self) -> Span {
        self.start.join(self.end)
    }
}

impl From<RangeSpan> for Span {
    fn from(value: RangeSpan) -> Self {
        value.span()
    }
}
