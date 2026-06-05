use crate::Span;

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct RangeSpan {
    start: Span,
    end: Span,
}

impl RangeSpan {
    #[inline]
    pub fn new(start: Span, end: Span) -> Self {
        Self { start, end }
    }

    #[inline]
    pub fn start(&self) -> Span {
        self.start
    }

    #[inline]
    pub fn end(&self) -> Span {
        self.end
    }

    #[inline]
    pub fn span(&self) -> Span {
        self.start.join(self.end)
    }
}

impl From<RangeSpan> for Span {
    #[inline]
    fn from(value: RangeSpan) -> Self {
        value.span()
    }
}
