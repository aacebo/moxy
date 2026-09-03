use moxy_token::{Span, TokenStream, TokenTree};

#[derive(Copy, Clone)]
pub struct Cursor<'a> {
    index: usize,
    tokens: &'a TokenStream,
}

impl<'a> Cursor<'a> {
    pub fn from_tokens(tokens: &'a TokenStream) -> Self {
        Self { index: 0, tokens }
    }

    pub fn is_empty(self) -> bool {
        self.index >= self.tokens.len()
    }

    pub fn span(self) -> Span {
        self.tokens.get(self.index).map(|t| t.span()).unwrap_or_default()
    }
}

impl<'a> Cursor<'a> {
    pub fn remaining(self) -> usize {
        self.tokens.len().saturating_sub(self.index)
    }

    pub fn next(self) -> Option<&'a TokenTree> {
        if self.index + 1 > self.tokens.len() - 1 {
            return None;
        }

        self.tokens.get(self.index + 1)
    }

    pub fn curr(self) -> Option<&'a TokenTree> {
        if self.index > self.tokens.len() - 1 {
            return None;
        }

        self.tokens.get(self.index)
    }

    pub fn prev(self) -> Option<&'a TokenTree> {
        if self.index - 1 < 0 {
            return None;
        }

        self.tokens.get(self.index - 1)
    }

    /// Look ahead `n` tokens without consuming (`nth(0)` == `curr`). When a glued
    /// punct has been split, the pending half is `nth(0)` and the real stream
    /// follows it.
    pub fn nth(self, n: usize) -> Option<&'a TokenTree> {
        if self.index + n > self.tokens.len() - 1 {
            return None;
        }

        self.tokens.get(self.index + n)
    }

    /// move the iterator forward and return the token.
    pub fn advance(self) -> Option<&'a TokenTree> {
        self.advance_by(1)?.first()
    }

    /// move the iterator forward by N and return the token.
    pub fn advance_by(mut self, n: usize) -> Option<&'a [TokenTree]> {
        if self.index + n > self.tokens.len() {
            return None;
        }

        let start = self.index;
        self.index += n;
        Some(&self.tokens[start..self.index])
    }

    /// Advance past tokens until `pred(curr())` is true (does NOT consume the matching token).
    /// Returns `self` for chaining.
    pub fn skip_until<F: Fn(Option<&TokenTree>) -> bool>(self, pred: F) -> Self {
        while !self.is_empty() && !pred(self.curr()) {
            self.advance();
        }

        self
    }
}
