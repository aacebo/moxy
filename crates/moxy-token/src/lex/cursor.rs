use crate::Span;
use crate::span::fallback;

use super::LexError;

/// Zero-copy immutable cursor over source text.
/// Each parse step returns a new advanced cursor.
/// A copyable cursor over source text during lexing.
#[derive(Copy, Clone)]
pub struct Cursor<'a> {
    rest: &'a str,
    offset: u32,
    generation: std::num::NonZeroU32,
}

impl<'a> Cursor<'a> {
    /// A cursor whose spans are in no source of the source map.
    pub fn new(src: &'a str, offset: u32) -> Self {
        Self::at(src, fallback::Span::new(offset, offset))
    }

    // A cursor at the start of `span`, whose spans take its generation
    pub(crate) fn at(src: &'a str, span: fallback::Span) -> Self {
        Self {
            rest: src,
            offset: span.byte_range().start as u32,
            generation: span.generation(),
        }
    }

    pub fn rest(&self) -> &'a str {
        self.rest
    }

    pub fn offset(&self) -> u32 {
        self.offset
    }

    pub fn is_empty(&self) -> bool {
        self.rest.is_empty()
    }

    pub fn first(&self) -> Option<char> {
        self.rest.chars().next()
    }

    pub fn starts_with(&self, s: &str) -> bool {
        self.rest.starts_with(s)
    }

    /// The span of the character at the cursor, which is empty at the end of
    /// the input.
    pub fn span(&self) -> Span {
        let width = self.first().map_or(0, char::len_utf8) as u32;
        fallback::Span::in_generation(self.generation, self.offset, self.offset + width).into()
    }

    /// Create a fallback::Span from this cursor to another.
    pub fn span_to(&self, end: &Cursor<'_>) -> Span {
        fallback::Span::in_generation(self.generation, self.offset, end.offset).into()
    }

    /// Get a slice of text from the current cursor to the provided end.
    pub fn slice_to(&self, end: Cursor<'_>) -> &'a str {
        let len = (end.offset - self.offset) as usize;
        &self.rest[..len]
    }

    /// Create an error at the current span
    pub fn error(&self) -> LexError {
        LexError::new(self.span())
    }

    /// Advance by 1 byte, which the offset counts.
    pub fn advance(&self) -> Self {
        self.advance_by(1)
    }

    /// Advance by `n` bytes, which the offset counts.
    pub fn advance_by(&self, n: usize) -> Self {
        Self {
            rest: &self.rest[n..],
            offset: self.offset + n as u32,
            generation: self.generation,
        }
    }

    /// Advance while predicate holds on chars.
    pub fn skip_while(&self, mut pred: impl FnMut(char) -> bool) -> Self {
        let mut bytes = 0;

        for ch in self.rest.chars() {
            if !pred(ch) {
                break;
            }

            bytes += ch.len_utf8();
        }

        self.advance_by(bytes)
    }

    pub fn skip_whitespace(mut self) -> Self {
        loop {
            // Whitespace
            let next = self.skip_while(|ch| ch.is_whitespace());

            if next.offset() != self.offset() {
                self = next;
                continue;
            }

            // Line comment — skip plain `//` and `////+`, but NOT doc `///`/`//!`.
            if self.starts_with("//") && !self.is_line_doc() {
                self = self.skip_while(|ch| ch != '\n');

                if self.starts_with("\n") {
                    self = self.advance();
                }

                continue;
            }

            // Block comment (nested) — skip plain `/*`, but NOT doc `/**`/`/*!`.
            if self.starts_with("/*") && !self.is_block_doc() {
                match self.skip_comment() {
                    None => break, // unterminated — let the main parser deal with it
                    Some(next) => {
                        self = next;
                        continue;
                    }
                }
            }

            break;
        }

        self
    }

    // Skips a leading byte order mark, which the compiler drops from every
    // source string, including one a procedural macro lexes
    pub(crate) fn skip_byte_order_mark(self) -> Self {
        if self.starts_with("\u{feff}") {
            self.advance_by('\u{feff}'.len_utf8())
        } else {
            self
        }
    }

    /// Skips what precedes the tokens of a source file, as the compiler does: a
    /// byte order mark, and a shebang line such as `#!/usr/bin/env -S cargo`,
    /// up to its line break. A `#!` that a `[` follows starts an inner
    /// attribute instead.
    pub fn skip_source_start(self) -> Self {
        let cursor = self.skip_byte_order_mark();

        if cursor.starts_with("#!") && !cursor.advance_by(2).skip_whitespace().starts_with("[") {
            return cursor.skip_while(|ch| ch != '\n');
        }

        cursor
    }

    /// True at a line doc comment: `///...` (but not `////...`) or `//!...`.
    pub fn is_line_doc(&self) -> bool {
        (self.starts_with("///") && !self.starts_with("////")) || self.starts_with("//!")
    }

    /// True at a block doc comment: `/**...` (but not `/***`/`/**/`) or `/*!...`.
    pub fn is_block_doc(&self) -> bool {
        self.starts_with("/*!") || (self.starts_with("/**") && !self.starts_with("/***") && !self.starts_with("/**/"))
    }

    /// If positioned at a doc comment, return `(cursor after it, is_inner, text)`.
    /// The text is kept as written, as the compiler gives it: only the line
    /// break that ends a line comment is left out.
    pub fn doc_comment(&self) -> Option<(Self, bool, String)> {
        if self.is_line_doc() {
            let inner = self.starts_with("//!");
            let body = self.advance_by(3); // skip /// or //!
            let end = body.skip_while(|ch| ch != '\n');
            let text = &body.rest()[..(end.offset() - body.offset()) as usize];
            let text = text.strip_suffix('\r').unwrap_or(text);
            let next = if end.starts_with("\n") { end.advance() } else { end };
            return Some((next, inner, text.to_string()));
        }

        if self.is_block_doc() {
            let inner = self.starts_with("/*!");
            let close = self.skip_comment()?;
            // `close` is just after the matching `*/`; the text is between the
            // opening `/**` or `/*!` and that `*/`
            let len = (close.offset() - self.offset()) as usize;
            let text = &self.rest()[3..len - 2];
            return Some((close, inner, text.to_string()));
        }

        None
    }

    pub fn skip_comment(&self) -> Option<Self> {
        let mut cur = self.advance_by(2); // skip /*
        let mut depth = 1u32;

        while !cur.is_empty() {
            if cur.starts_with("/*") {
                depth += 1;
                cur = cur.advance_by(2);
            } else if cur.starts_with("*/") {
                depth -= 1;
                cur = cur.advance_by(2);

                if depth == 0 {
                    return Some(cur);
                }
            } else {
                let ch = cur.first().unwrap();
                cur = cur.advance_by(ch.len_utf8());
            }
        }

        None
    }
}
