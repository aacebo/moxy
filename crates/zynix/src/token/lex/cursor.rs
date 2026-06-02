use super::LexError;
use crate::Span;
use crate::span::fallback;

/// Zero-copy immutable cursor over source text.
/// Each parse step returns a new advanced cursor.
#[derive(Copy, Clone)]
pub struct Cursor<'a> {
    rest: &'a str,
    off: u32,
}

impl<'a> Cursor<'a> {
    pub fn new(src: &'a str, offset: u32) -> Self {
        Self { rest: src, off: offset }
    }

    pub fn rest(&self) -> &'a str {
        self.rest
    }

    pub fn offset(&self) -> u32 {
        self.off
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

    /// Create a fallback::Span from this cursor to another.
    pub fn span_to(&self, end: &Cursor<'_>) -> Span {
        fallback::Span::new(self.off, end.off).into()
    }

    pub fn span(&self) -> Span {
        fallback::Span::new(self.off, self.off + 1).into()
    }

    pub fn error(&self) -> LexError {
        LexError::new(self.span())
    }

    /// Advance by `n` bytes, counting characters for the offset.
    pub fn advance(&self, n: usize) -> Self {
        let consumed = &self.rest[..n];
        let chars = consumed.chars().count() as u32;

        Self {
            rest: &self.rest[n..],
            off: self.off + chars,
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

        self.advance(bytes)
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
                    self = self.advance(1);
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
            let body = self.advance(3); // skip /// or //!
            let end = body.skip_while(|ch| ch != '\n');
            let text: String = body.rest()[..(end.offset() - body.offset()) as usize].to_string();
            let next = if end.starts_with("\n") { end.advance(1) } else { end };
            return Some((next, inner, text.trim().to_string()));
        }
        if self.is_block_doc() {
            let inner = self.starts_with("/*!");
            let body = self.advance(3); // skip /** or /*!
            let close = body.skip_comment_to_close()?;
            // close is positioned just after `*/`; text is between body and `*/`.
            let len = (close.offset() - body.offset()) as usize - 2;
            let text: String = body.rest()[..len].to_string();
            return Some((close, inner, text.trim().to_string()));
        }
        None
    }

    /// Skip to just past the matching `*/` of a (non-nested) block comment body.
    fn skip_comment_to_close(&self) -> Option<Self> {
        let mut cur = *self;
        while !cur.is_empty() {
            if cur.starts_with("*/") {
                return Some(cur.advance(2));
            }
            let ch = cur.first().unwrap();
            cur = cur.advance(ch.len_utf8());
        }
        None
    }

    pub fn skip_comment(&self) -> Option<Self> {
        let mut cur = self.advance(2); // skip /*
        let mut depth = 1u32;

        while !cur.is_empty() {
            if cur.starts_with("/*") {
                depth += 1;
                cur = cur.advance(2);
            } else if cur.starts_with("*/") {
                depth -= 1;
                cur = cur.advance(2);

                if depth == 0 {
                    return Some(cur);
                }
            } else {
                let ch = cur.first().unwrap();
                cur = cur.advance(ch.len_utf8());
            }
        }

        None
    }
}
