mod delim;
pub(crate) mod fallback;
mod range;

pub use delim::*;
pub use range::*;

use crate::source::Location;

#[derive(Debug, Copy, Clone)]
pub enum Span {
    Compiler(proc_macro::Span),
    Fallback(fallback::Span),
}

impl Span {
    pub fn call_site() -> Self {
        if proc_macro::is_available() {
            Self::Compiler(proc_macro::Span::call_site())
        } else {
            Self::Fallback(fallback::Span::call_site())
        }
    }

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

    pub fn len(&self) -> usize {
        match self {
            Self::Compiler(_) => 0,
            Self::Fallback(v) => v.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Self::Compiler(_) => true,
            Self::Fallback(v) => v.is_empty(),
        }
    }

    pub fn contains(&self, i: usize) -> bool {
        match self {
            Self::Compiler(_) => false,
            Self::Fallback(v) => v.contains(i),
        }
    }

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

impl From<proc_macro::Span> for Span {
    fn from(value: proc_macro::Span) -> Self {
        Self::Compiler(value)
    }
}

impl From<fallback::Span> for Span {
    fn from(value: fallback::Span) -> Self {
        Self::Fallback(value)
    }
}

impl From<Span> for proc_macro::Span {
    fn from(value: Span) -> Self {
        match value {
            Span::Compiler(s) => s,
            Span::Fallback(_) => proc_macro::Span::call_site(),
        }
    }
}

#[cfg(nightly)]
impl proc_macro::MultiSpan for Span {
    fn into_spans(self) -> Vec<proc_macro::Span> {
        match self {
            Self::Compiler(s) => vec![s],
            Self::Fallback(_) => vec![proc_macro::Span::call_site()],
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    fn fb(start: u32, end: u32) -> Span {
        crate::source::SourceMap::with_mut(|sm| {
            if sm.is_empty() {
                sm.push("0123456789abcdef");
            }
        });
        Span::Fallback(fallback::Span::new(start, end))
    }

    #[test]
    fn call_site_does_not_panic() {
        let _ = Span::call_site();
    }

    #[test]
    fn mixed_site_does_not_panic() {
        let _ = Span::mixed_site();
    }

    #[test]
    fn def_site_does_not_panic() {
        let _ = Span::def_site();
    }

    #[test]
    fn default_does_not_panic() {
        let _ = Span::default();
    }

    #[test]
    fn start_end_fallback() {
        use crate::source::SourceMap;
        // Register source so span resolution works
        SourceMap::with_mut(|sm| {
            sm.push("0123456789abcdef");
        });
        let s = fb(5, 10);
        assert_eq!(s.start().index(), 5);
        assert_eq!(s.end().index(), 10);
    }

    #[test]
    fn len_fallback() {
        assert_eq!(fb(0, 7).len(), 7);
        assert_eq!(fb(3, 3).len(), 0);
    }

    #[test]
    fn is_empty_fallback() {
        assert!(fb(0, 0).is_empty());
        assert!(!fb(0, 1).is_empty());
    }

    #[test]
    fn contains_fallback() {
        let s = fb(2, 5);
        assert!(!s.contains(1));
        assert!(s.contains(2));
        assert!(s.contains(4));
        assert!(!s.contains(5));
    }

    #[test]
    fn is_subset_fallback() {
        let outer = fb(0, 10);
        let inner = fb(2, 5);
        assert!(inner.is_subset(&outer));
        assert!(!outer.is_subset(&inner));
    }

    #[test]
    fn join_fallback() {
        let a = fb(2, 5);
        let b = fb(8, 12);
        let joined = a.join(b);
        match joined {
            Span::Fallback(v) => {
                assert_eq!(v.byte_range(), 2..12);
            }
            _ => panic!("expected fallback"),
        }
    }

    #[test]
    fn from_fallback_roundtrip() {
        let inner = fallback::Span::new(3, 7);
        let wrapper: Span = inner.into();
        assert!(matches!(wrapper, Span::Fallback(v) if v == inner));
    }

    #[test]
    fn eq_fallback() {
        // On stable, all spans are equal
        let a = Span::default();
        let b = Span::default();
        assert_eq!(a, b);
    }
}
