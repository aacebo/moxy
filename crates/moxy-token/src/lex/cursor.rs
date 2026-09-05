use crate::Span;
use crate::span::fallback;

use super::LexError;

/// Zero-copy immutable cursor over source text.
/// Each parse step returns a new advanced cursor.
#[derive(Copy, Clone)]
pub struct Cursor<'a> {
    rest: &'a str,
    offset: u32,
}

impl<'a> Cursor<'a> {
    pub fn new(src: &'a str, offset: u32) -> Self {
        Self { rest: src, offset }
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

    pub fn span(&self) -> Span {
        fallback::Span::new(self.offset, self.offset + 1).into()
    }

    /// Create a fallback::Span from this cursor to another.
    pub fn span_to(&self, end: &Cursor<'_>) -> Span {
        fallback::Span::new(self.offset, end.offset).into()
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

    /// Advance by 1 byte, counting characters for the offset.
    pub fn advance(&self) -> Self {
        self.advance_by(1)
    }

    /// Advance by `n` bytes, counting characters for the offset.
    pub fn advance_by(&self, n: usize) -> Self {
        Self {
            rest: &self.rest[n..],
            offset: self.offset + n as u32,
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

    /// True at a line doc comment: `///...` (but not `////...`) or `//!...`.
    pub fn is_line_doc(&self) -> bool {
        (self.starts_with("///") && !self.starts_with("////")) || self.starts_with("//!")
    }

    /// True at a block doc comment: `/**...` (but not `/***`/`/**/`) or `/*!...`.
    pub fn is_block_doc(&self) -> bool {
        self.starts_with("/*!") || (self.starts_with("/**") && !self.starts_with("/***") && !self.starts_with("/**/"))
    }

    /// If positioned at a doc comment, return `(cursor after it, is_inner, text)`.
    pub fn doc_comment(&self) -> Option<(Self, bool, String)> {
        if self.is_line_doc() {
            let inner = self.starts_with("//!");
            let body = self.advance_by(3); // skip /// or //!
            let end = body.skip_while(|ch| ch != '\n');
            let text: String = body.rest()[..(end.offset() - body.offset()) as usize].to_string();
            let next = if end.starts_with("\n") { end.advance() } else { end };
            return Some((next, inner, text.trim().to_string()));
        }

        if self.is_block_doc() {
            let inner = self.starts_with("/*!");
            let body = self.advance_by(3); // skip /** or /*!
            let close = body.skip_comment_to_close()?;
            // close is positioned just after `*/`; text is between body and `*/`.
            let len = (close.offset() - body.offset()) as usize - 2;
            let text: String = body.rest()[..len].to_string();
            return Some((close, inner, text.trim().to_string()));
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

    /// Skip to just past the matching `*/` of a (non-nested) block comment body.
    fn skip_comment_to_close(&self) -> Option<Self> {
        let mut cur = *self;

        while !cur.is_empty() {
            if cur.starts_with("*/") {
                return Some(cur.advance_by(2));
            }

            let ch = cur.first().unwrap();
            cur = cur.advance_by(ch.len_utf8());
        }

        None
    }
}
