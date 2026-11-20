use std::num::NonZeroU32;

use crate::source::{Location, SourceMap};

/// A source range associated with a token.
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Span {
    /// the source map generation the positions belong to, or
    /// [`Span::NO_SOURCE`]. Being nonzero, it leaves the compiler span of
    /// [`crate::Span`] room beside it, so neither span grows.
    #[cfg_attr(feature = "serde", serde(skip))]
    generation: NonZeroU32,

    /// the start position (inclusive).
    start: u32,

    /// the end position (exclusive).
    end: u32,
}

impl Span {
    // The generation of positions in no source of the source map, which
    // numbers its generations from the next one
    pub(crate) const NO_SOURCE: NonZeroU32 = NonZeroU32::MIN;

    /// A span of positions in no source of the source map.
    pub const fn new(start: u32, end: u32) -> Self {
        Self::in_generation(Self::NO_SOURCE, start, end)
    }

    pub(crate) const fn in_generation(generation: NonZeroU32, start: u32, end: u32) -> Self {
        Self { generation, start, end }
    }

    pub(crate) const fn generation(&self) -> NonZeroU32 {
        self.generation
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

    /// The location of the start in its source. A span whose source is not in
    /// the source map, as the call site or a span lexed inside a released
    /// [`SourceScope`](crate::source::SourceScope), is at line 0 and column 0.
    pub fn start(&self) -> Location {
        self.locate(self.start)
    }

    /// The location of the end in its source, as [`Span::start`] finds it.
    pub fn end(&self) -> Location {
        self.locate(self.end)
    }

    fn locate(&self, position: u32) -> Location {
        SourceMap::with(|sm| match sm.find(*self) {
            Some(file) => file.location(position as usize),
            None => Location::new(0, 0, 0),
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
        self.generation.get() == other.generation.get() && self.start >= other.start && self.end <= other.end
    }

    // Positions of two generations do not compare, so joining them keeps the
    // span that has a source
    pub const fn join(self, other: Self) -> Self {
        if self.generation.get() != other.generation.get() {
            return if self.generation.get() == Self::NO_SOURCE.get() {
                other
            } else {
                self
            };
        }

        let start = if self.start < other.start { self.start } else { other.start };
        let end = if self.end > other.end { self.end } else { other.end };
        Self {
            generation: self.generation,
            start,
            end,
        }
    }
}

impl Default for Span {
    fn default() -> Self {
        Self::call_site()
    }
}
