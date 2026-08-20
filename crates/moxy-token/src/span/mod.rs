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

                if cfg!(nightly) {
                    Location::new(v.byte_range().start, lc.line(), lc.column())
                } else {
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

                if cfg!(nightly) {
                    Location::new(v.byte_range().end, lc.line(), lc.column())
                } else {
                    Location::new(0, lc.line(), lc.column())
                }
            }
            Self::Fallback(v) => v.end(),
        }
    }

    pub fn byte_range(&self) -> std::ops::Range<usize> {
        match self {
            Self::Compiler(v) => {
                if cfg!(nightly) {
                    v.byte_range()
                } else {
                    0..0
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
            Self::Compiler(s) => {
                let range = s.byte_range();
                range.start.hash(state);
                range.end.hash(state);
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
            Self::Compiler(v) => {
                let mut o = s.serialize_struct("Span", 2)?;
                o.serialize_field("start", &v.byte_range().start)?;
                o.serialize_field("end", &v.byte_range().end)?;
                o.end()
            }
        }
    }
}
