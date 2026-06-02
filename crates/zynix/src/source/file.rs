use std::cell::RefCell;
use std::collections::BTreeMap;

use super::Location;
use crate::span::fallback::Span;

/// Primarily used to map spans (0 based character index ranges)
/// to bytes.
#[derive(Debug)]
pub struct Source {
    /// raw source text
    text: String,

    /// file-wide lo..hi in proc-macro2 space
    span: Span,

    /// line start offsets, in char units
    lines: Vec<usize>,

    /// Cache mapping character indices to UTF-8 byte offsets for efficient span slicing
    char_to_byte: RefCell<BTreeMap<usize, usize>>,
}

impl Source {
    pub(crate) fn new(start: usize, src: impl Into<String>) -> Self {
        let text = src.into();
        let mut lines = vec![0];
        let mut total = 0usize;

        for ch in text.chars() {
            total += 1;

            if ch == '\n' {
                lines.push(total);
            }
        }

        let mut char_to_byte = BTreeMap::new();
        char_to_byte.insert(0, 0);

        Self {
            text,
            span: Span::new(start as u32, (start + total) as u32),
            lines,
            char_to_byte: RefCell::new(char_to_byte),
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn span(&self) -> Span {
        self.span
    }

    /// Resolves the given span into a byte index range.
    pub fn range(&self, span: Span) -> std::ops::Range<usize> {
        let r = span.byte_range();
        self.byte(r.start)..self.byte(r.end)
    }

    /// Gets a sub span of source text from the file.
    pub fn slice(&self, span: Span) -> String {
        self.text[self.range(span)].to_owned()
    }

    /// Resolves a global character index within this file into a 0-based `Location`.
    pub fn location(&self, i: usize) -> Location {
        let index = i - self.span.byte_range().start;

        match self.lines.binary_search(&index) {
            Err(next) => Location::new(index, next - 1, index - self.lines[next - 1]),
            Ok(line) => Location::new(index, line, 0),
        }
    }

    /// Returns the UTF-8 byte index corresponding to a global character index.
    pub fn byte(&self, i: usize) -> usize {
        let index = i - self.span.byte_range().start;
        let mut cache = self.char_to_byte.borrow_mut();

        if let Some(byte_index) = cache.get(&index) {
            return *byte_index;
        }

        let (&ci, &bi) = cache.range(..=index).next_back().unwrap();

        let mut char_index = ci;
        let mut byte_index = bi;

        #[allow(clippy::explicit_counter_loop)]
        for ch in self.text[bi..].chars() {
            if char_index == index {
                cache.insert(index, byte_index);
                return byte_index;
            }

            char_index += 1;
            byte_index += ch.len_utf8();
        }

        cache.insert(index, byte_index);
        byte_index
    }
}

impl Default for Source {
    fn default() -> Self {
        Self {
            text: String::default(),
            span: Span::default(),
            lines: vec![0],
            char_to_byte: RefCell::new(BTreeMap::default()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn span_covers_full_text() {
        let src = Source::new(0, "hello");
        assert_eq!(src.span(), Span::new(0, 5));
    }

    #[test]
    fn span_with_offset() {
        let src = Source::new(10, "abc");
        assert_eq!(src.span(), Span::new(10, 13));
    }

    #[test]
    fn slice_returns_substring() {
        let src = Source::new(0, "hello world");
        assert_eq!(src.slice(Span::new(0, 5)), "hello");
        assert_eq!(src.slice(Span::new(6, 11)), "world");
    }

    #[test]
    fn location_first_line() {
        let src = Source::new(0, "hello\nworld");
        let loc = src.location(0);
        assert_eq!(loc.line(), 0);
        assert_eq!(loc.column(), 0);
    }

    #[test]
    fn location_second_line() {
        let src = Source::new(0, "hello\nworld");
        // char index 6 = 'w' on second line
        let loc = src.location(6);
        assert_eq!(loc.line(), 1);
        assert_eq!(loc.column(), 0);
    }

    #[test]
    fn location_mid_line() {
        let src = Source::new(0, "hello\nworld");
        let loc = src.location(8);
        assert_eq!(loc.line(), 1);
        assert_eq!(loc.column(), 2);
    }

    #[test]
    fn byte_ascii() {
        let src = Source::new(0, "hello");
        assert_eq!(src.byte(0), 0);
        assert_eq!(src.byte(3), 3);
    }

    #[test]
    fn byte_multibyte_utf8() {
        // 'é' is 2 bytes in UTF-8
        let src = Source::new(0, "café");
        // chars: c(0) a(1) f(2) é(3)
        // bytes: c(0) a(1) f(2) é(3..4)
        assert_eq!(src.byte(3), 3);
        assert_eq!(src.byte(4), 5); // after é (2 bytes)
    }

    #[test]
    fn text_returns_source() {
        let src = Source::new(0, "fn main() {}");
        assert_eq!(src.text(), "fn main() {}");
    }
}
