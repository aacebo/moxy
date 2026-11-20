use std::cell::RefCell;
use std::marker::PhantomData;
use std::num::NonZeroU32;

use super::Source;
use crate::span::fallback::Span;

thread_local! {
    static GLOBAL: RefCell<SourceMap> = RefCell::new(SourceMap::new());
}

/// A collection of source files used to resolve token locations.
///
/// Each thread has one map, which holds every string a token stream is lexed
/// from on that thread until a [`SourceScope`] made before it is dropped.
/// Positions are never given back: a released source's positions stay unused,
/// and when the positions run out a new generation of them starts, so a span
/// of a released source finds no source, never a later one.
#[derive(Debug)]
pub struct SourceMap {
    sources: Vec<Source>,
    generation: NonZeroU32,
    // The first position the next source can take, which the arithmetic keeps
    // in 64 bits so it cannot wrap on a 32-bit target
    next: u64,
}

impl SourceMap {
    pub fn new() -> Self {
        Self {
            sources: vec![],
            generation: Span::NO_SOURCE.saturating_add(1),
            next: 0,
        }
    }

    pub fn with<R>(f: impl FnOnce(&Self) -> R) -> R {
        GLOBAL.with(|sm| f(&sm.borrow()))
    }

    pub fn with_mut<R>(f: impl FnOnce(&mut Self) -> R) -> R {
        GLOBAL.with(|sm| f(&mut sm.borrow_mut()))
    }

    /// Makes a scope that releases, when it is dropped, every source lexed on
    /// this thread since it was made.
    pub fn scope() -> SourceScope {
        SourceScope {
            mark: Self::with(|sm| (sm.generation, sm.next)),
            thread: PhantomData,
        }
    }

    pub fn as_slice(&self) -> &[Source] {
        &self.sources
    }

    pub fn len(&self) -> usize {
        self.sources.len()
    }

    pub fn is_empty(&self) -> bool {
        self.sources.is_empty()
    }

    pub fn find(&self, span: Span) -> Option<&Source> {
        match self.find_index(span) {
            None => None,
            Some(i) => Some(&self.sources[i]),
        }
    }

    pub fn find_mut(&mut self, span: Span) -> Option<&mut Source> {
        match self.find_index(span) {
            None => None,
            Some(i) => Some(&mut self.sources[i]),
        }
    }

    pub fn find_path(&self, span: Span) -> Option<String> {
        if let Some(i) = self.find_index(span) {
            return if i == 0 {
                Some("<unspecified>".to_owned())
            } else {
                Some(format!("<parsed string {}>", i))
            };
        }

        None
    }

    /// The index of the source that holds the whole span, if any.
    pub fn find_index(&self, span: Span) -> Option<usize> {
        let sr = span.byte_range();
        let index = self
            .sources
            .binary_search_by(|src| {
                let fr = src.span().byte_range();

                src.span().generation().cmp(&span.generation()).then(if fr.end < sr.start {
                    std::cmp::Ordering::Less
                } else if fr.start > sr.end {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Equal
                })
            })
            .ok()?;

        let fr = self.sources[index].span().byte_range();
        (fr.start <= sr.start && sr.end <= fr.end).then_some(index)
    }

    /// Adds a source and returns the positions it takes.
    ///
    /// # Panics
    ///
    /// Panics on a text of `u32::MAX` bytes or more, which 32-bit positions
    /// cannot hold with the last one free; the lexer refuses such a text
    /// before it gets here.
    pub fn push(&mut self, src: impl Into<String>) -> Span {
        let text = src.into();
        let len = u64::try_from(text.len()).unwrap_or(u64::MAX);
        assert!(
            len < u64::from(u32::MAX),
            "a source of `u32::MAX` bytes or more has no 32-bit positions"
        );

        // The last position stays free, so a span one past the end of a
        // source still fits
        if self.next + len >= u64::from(u32::MAX) {
            self.generation = self.generation.saturating_add(1);
            self.next = 0;
        }

        let src = Source::new(self.generation, self.next as usize, text);
        let span = src.span();
        // One position between sources keeps a span that ends one source,
        // such as the end of its input, out of the next
        self.next = span.byte_range().end as u64 + 1;
        self.sources.push(src);
        span
    }
}

impl Default for SourceMap {
    fn default() -> Self {
        Self::new()
    }
}

/// A scope over the sources lexed on one thread, made by
/// [`SourceMap::scope`].
///
/// Dropping it releases every source lexed on the thread since it was made,
/// so a long-running caller that lexes in a loop can keep the map from
/// growing. A span of a released source, held by a token that outlives the
/// scope, finds no source: its [`start`](crate::Span::start) and
/// [`end`](crate::Span::end) are at line 0 and column 0, and
/// [`SourceMap::find`] returns `None`. That includes the tokens of a
/// `template!` literal that were first lexed inside the scope, which each
/// thread lexes once and keeps.
///
/// A scope releases what was lexed after it was made, so scopes may drop in
/// any order: an inner scope releases only what was lexed inside it, and an
/// outer one dropped first releases the inner one's sources as well. A scope
/// dropped while the map is borrowed, as inside [`SourceMap::with`], or while
/// its thread ends, releases nothing.
#[must_use = "a scope releases its sources when it is dropped"]
#[derive(Debug)]
pub struct SourceScope {
    // The generation and the next position when the scope was made
    mark: (NonZeroU32, u64),
    // The map is the thread's own, so the scope stays on that thread
    thread: PhantomData<*const ()>,
}

impl Drop for SourceScope {
    fn drop(&mut self) {
        let _ = GLOBAL.try_with(|sm| {
            if let Ok(mut sm) = sm.try_borrow_mut() {
                let kept = sm.sources.partition_point(|src| {
                    let span = src.span();
                    (span.generation(), span.byte_range().start as u64) < self.mark
                });
                sm.sources.truncate(kept);
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Positions that would end at the last 32-bit position start a new
    // generation, and a span of a released source finds nothing even where a
    // source of the new generation takes the same positions
    #[test]
    fn positions_start_a_new_generation_before_they_run_out() {
        let mut map = SourceMap::new();
        let old = map.push("abcdef");
        map.next = (u32::MAX - 6).into();
        let new = map.push("abcdef");
        map.sources.remove(0);

        assert_eq!(new.generation().get(), old.generation().get() + 1);
        assert_eq!(new.byte_range(), 0..6);
        assert_eq!(old.byte_range(), 0..6);
        assert!(map.find(old).is_none());
        assert!(map.find(new).is_some());
    }
}
