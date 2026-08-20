use std::cell::RefCell;

use super::Source;
use crate::span::fallback::Span;

thread_local! {
    static GLOBAL: RefCell<SourceMap> = RefCell::new(SourceMap::new());
}

#[derive(Debug, Default)]
pub struct SourceMap(Vec<Source>);

impl SourceMap {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn with<R>(f: impl FnOnce(&Self) -> R) -> R {
        GLOBAL.with(|sm| f(&sm.borrow()))
    }

    pub fn with_mut<R>(f: impl FnOnce(&mut Self) -> R) -> R {
        GLOBAL.with(|sm| f(&mut sm.borrow_mut()))
    }

    pub fn files(&self) -> &[Source] {
        &self.0
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn find(&self, span: Span) -> Option<&Source> {
        match self.find_index(span) {
            None => None,
            Some(i) => Some(&self.0[i]),
        }
    }

    pub fn find_mut(&mut self, span: Span) -> Option<&mut Source> {
        match self.find_index(span) {
            None => None,
            Some(i) => Some(&mut self.0[i]),
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

    pub fn find_index(&self, span: Span) -> Option<usize> {
        let sr = span.byte_range();
        self.0
            .binary_search_by(|file| {
                let fr = file.span().byte_range();

                if fr.end < sr.start {
                    std::cmp::Ordering::Less
                } else if fr.start > sr.end {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Equal
                }
            })
            .ok()
    }

    pub fn push(&mut self, src: impl Into<String>) -> Span {
        let start = self.0.last().map(|file| file.span().byte_range().end).unwrap_or(0);
        let file = Source::new(start, src);
        let span = file.span();
        self.0.push(file);
        span
    }
}
