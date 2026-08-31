mod delim;
pub(crate) mod fallback;
mod range;

#[doc(inline)]
pub use delim::*;
#[doc(inline)]
pub use range::*;

use crate::source::Location;

pub trait Spanner {
    fn span(&self) -> Span;
}

#[derive(Debug, Copy, Clone)]
pub enum Span {
    Compiler(proc_macro::Span),
    Fallback(fallback::Span),
}

impl Span {
    #[inline]
    pub fn call_site() -> Self {
        if proc_macro::is_available() {
            Self::Compiler(proc_macro::Span::call_site())
        } else {
            Self::Fallback(fallback::Span::call_site())
        }
    }

    #[inline]
    pub fn mixed_site() -> Self {
        if proc_macro::is_available() {
            Self::Compiler(proc_macro::Span::mixed_site())
        } else {
            Self::Fallback(fallback::Span::mixed_site())
        }
    }

    #[inline]
    pub fn def_site() -> Self {
        #[cfg(nightly)]
        if proc_macro::is_available() {
            return Self::Compiler(proc_macro::Span::def_site());
        }

        Self::Fallback(fallback::Span::def_site())
    }

    pub fn start(&self) -> Location {
        match self {
            Self::Compiler(v) => {
                let lc = v.start();

                #[cfg(nightly)]
                {
                    Location::new(v.byte_range().start, lc.line(), lc.column())
                }

                #[cfg(not(nightly))]
                {
                    Location::new(0, lc.line(), lc.column())
                }
            }
            Self::Fallback(v) => v.start(),
        }
    }

    pub fn end(&self) -> Location {
        match self {
            Self::Compiler(v) => {
                let lc = v.end();

                #[cfg(nightly)]
                {
                    Location::new(v.byte_range().end, lc.line(), lc.column())
                }

                #[cfg(not(nightly))]
                {
                    let src = v.source_text().unwrap_or_default();
                    let index = line_column_offset(&src, v.line(), v.column());
                    Location::new(index, lc.line(), lc.column())
                }
            }
            Self::Fallback(v) => v.end(),
        }
    }

    pub fn byte_range(&self) -> std::ops::Range<usize> {
        match self {
            #[allow(unused)]
            Self::Compiler(v) => {
                #[cfg(nightly)]
                {
                    v.byte_range()
                }

                #[cfg(not(nightly))]
                {
                    let start = v.start();
                    let end = v.end();
                    let src = v.source_text().unwrap_or_default();
                    let start = line_column_offset(&src, start.line(), start.column());
                    let end = line_column_offset(&src, end.line(), end.column());
                    start..end
                }
            }
            Self::Fallback(v) => v.byte_range(),
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        match self {
            Self::Compiler(_) => 0,
            Self::Fallback(v) => v.len(),
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Compiler(_) => true,
            Self::Fallback(v) => v.is_empty(),
        }
    }

    #[inline]
    pub fn contains(&self, i: usize) -> bool {
        match self {
            Self::Compiler(_) => false,
            Self::Fallback(v) => v.contains(i),
        }
    }

    #[inline]
    pub fn is_subset(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Fallback(a), Self::Fallback(b)) => a.is_subset(b),
            _ => false,
        }
    }

    #[inline]
    pub fn join(&self, other: Self) -> Self {
        #[cfg(nightly)]
        if let (Self::Compiler(a), Self::Compiler(b)) = (self, other) {
            if let Some(joined) = a.join(b) {
                return Self::Compiler(joined);
            }
        }

        if let (Self::Fallback(a), Self::Fallback(b)) = (self, other) {
            return Self::Fallback(a.join(b));
        }

        other
    }

    /// Split a span at `head_len` characters from its start, returning
    /// `(head_span, rest_span)`. Only `Fallback` spans carry offsets we can split;
    /// for compiler spans we reuse the whole span for both halves.
    #[inline]
    pub fn split(self, at: usize) -> (Span, Span) {
        match self {
            Self::Fallback(s) => {
                let range = s.byte_range();
                let mid = (range.start + at) as u32;
                let head = fallback::Span::new(range.start as u32, mid);
                let rest = fallback::Span::new(mid, range.end as u32);
                (Span::Fallback(head), Span::Fallback(rest))
            }
            other => (other, other),
        }
    }
}

impl Default for Span {
    #[inline]
    fn default() -> Self {
        Self::call_site()
    }
}

impl Eq for Span {}

impl PartialEq for Span {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Compiler(a), Self::Compiler(b)) => {
                a.start().line() == b.start().line()
                    && a.start().column() == b.start().column()
                    && a.end().line() == b.end().line()
                    && a.end().column() == b.end().column()
            }
            (Self::Fallback(a), Self::Fallback(b)) => a == b,
            _ => false,
        }
    }
}

impl Ord for Span {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Self::Compiler(a), Self::Compiler(b)) => match a.start().line().cmp(&b.start().line()) {
                std::cmp::Ordering::Equal => match a.start().column().cmp(&b.start().column()) {
                    std::cmp::Ordering::Equal => match a.end().line().cmp(&b.end().line()) {
                        std::cmp::Ordering::Equal => a.end().column().cmp(&b.end().column()),
                        ord => ord,
                    },
                    ord => ord,
                },
                ord => ord,
            },
            (Self::Fallback(a), Self::Fallback(b)) => a.cmp(b),
            (Self::Fallback(_), Self::Compiler(_)) => std::cmp::Ordering::Less,
            (Self::Compiler(_), Self::Fallback(_)) => std::cmp::Ordering::Greater,
        }
    }
}

impl PartialOrd for Span {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::hash::Hash for Span {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            #[allow(unused)]
            Self::Compiler(s) => {
                #[cfg(nightly)]
                {
                    let range = s.byte_range();
                    range.start.hash(state);
                    range.end.hash(state);
                }

                #[cfg(not(nightly))]
                {
                    let start = s.start();
                    let end = s.end();
                    let src = s.source_text().unwrap_or_default();

                    line_column_offset(&src, start.line(), start.column()).hash(state);
                    line_column_offset(&src, end.line(), end.column()).hash(state);
                }
            }
            Self::Fallback(s) => s.hash(state),
        }
    }
}

impl From<fallback::Span> for Span {
    #[inline]
    fn from(value: fallback::Span) -> Self {
        Self::Fallback(value)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Span {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        match self {
            Self::Fallback(v) => v.serialize(s),
            #[allow(unused)]
            Self::Compiler(v) => {
                let mut o = s.serialize_struct("Span", 2)?;

                #[cfg(nightly)]
                {
                    o.serialize_field("start", &v.byte_range().start)?;
                    o.serialize_field("end", &v.byte_range().end)?;
                }

                #[cfg(not(nightly))]
                {
                    let start = v.start();
                    let end = v.end();
                    let src = v.source_text().unwrap_or_default();

                    o.serialize_field("start", &line_column_offset(&src, start.line(), start.column()))?;
                    o.serialize_field("end", &line_column_offset(&src, end.line(), end.column()))?;
                }

                o.end()
            }
        }
    }
}

pub fn line_column_offset(src: &str, ln: usize, col: usize) -> usize {
    let mut curr = 0;

    for (i, line) in src.lines().enumerate() {
        if i + 1 == ln {
            let offset = line.char_indices().nth(col).map(|(i, _)| i).unwrap_or(line.len());
            return curr + offset;
        }

        curr += line.len() + 1;
    }

    0
}
