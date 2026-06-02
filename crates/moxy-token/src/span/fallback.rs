use crate::source::{Location, SourceMap};

#[derive(Debug, Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Span {
    /// the start char index (inclusive).
    start: u32,

    /// the end char index (exclusive).
    end: u32,
}

impl Span {
    pub const fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }

    pub const fn call_site() -> Self {
        Self::new(0, 0)
    }

    pub const fn mixed_site() -> Self {
        Self::call_site()
    }

    pub const fn def_site() -> Self {
        Self::call_site()
    }

    pub fn byte_range(&self) -> std::ops::Range<usize> {
        self.start as usize..self.end as usize
    }

    pub fn start(&self) -> Location {
        SourceMap::with(|sm| {
            let file = sm.find(*self).expect("span not found in source map");
            file.location(self.start as usize)
        })
    }

    pub fn end(&self) -> Location {
        SourceMap::with(|sm| {
            let file = sm.find(*self).expect("span not found in source map");
            file.location(self.end as usize)
        })
    }

    pub const fn len(&self) -> usize {
        (self.end - self.start) as usize
    }

    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub const fn contains(&self, i: usize) -> bool {
        i >= self.start as usize && i < self.end as usize
    }

    pub const fn is_subset(&self, other: &Self) -> bool {
        self.start >= other.start && self.end <= other.end
    }

    pub const fn join(self, other: Self) -> Self {
        let start = if self.start < other.start { self.start } else { other.start };

        let end = if self.end > other.end { self.end } else { other.end };

        Self { start, end }
    }
}

impl From<proc_macro::Span> for Span {
    fn from(value: proc_macro::Span) -> Self {
        if cfg!(nightly) {
            let r = value.byte_range();
            return Self::new(r.start as u32, r.end as u32);
        }

        Self::default()
    }
}

impl From<Span> for proc_macro::Span {
    fn from(_value: Span) -> Self {
        proc_macro::Span::call_site()
    }
}
