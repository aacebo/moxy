use moxy_token::{Span, TokenStream, TokenTree};

/// Copyable transactional position within a token parser.
#[derive(Copy, Clone)]
pub struct Cursor<'a> {
    index: usize,
    tokens: &'a TokenStream,
}

impl<'a> Cursor<'a> {
    pub fn from_tokens(tokens: &'a TokenStream) -> Self {
        Self {
            index: 0,
            tokens,
        }
    }

    pub fn is_empty(self) -> bool {
        self.index >= self.tokens.len()
    }

    pub fn span(self) -> Span {
        self.tokens.get(self.index).map(TokenTree::span).unwrap_or_default()
    }

    pub fn remaining(self) -> usize {
        self.tokens.len().saturating_sub(self.index)
    }

    pub fn curr(self) -> Option<&'a TokenTree> {
        self.nth(0)
    }

    pub fn next(self) -> Option<&'a TokenTree> {
        self.nth(1)
    }

    pub fn nth(self, n: usize) -> Option<&'a TokenTree> {
        self.tokens.get(self.index.checked_add(n)?)
    }

    pub fn prev(self) -> Option<&'a TokenTree> {
        self.tokens.get(self.index.checked_sub(1)?)
    }

    pub fn advance(mut self) -> (Self, Option<&'a TokenTree>) {
        let token = self.tokens.get(self.index);

        if token.is_some() {
            self.index += 1;
        }

        (self, token)
    }

    pub fn advance_by(mut self, n: usize) -> (Self, Option<&'a [TokenTree]>) {
        let Some(end) = self.index.checked_add(n) else {
            return (self, None);
        };

        let tokens = self.tokens.get(self.index..end);

        if tokens.is_some() {
            self.index = end;
        }

        (self, tokens)
    }

    pub fn skip_until<F: Fn(Option<&TokenTree>) -> bool>(mut self, pred: F) -> Self {
        while !self.is_empty() && !pred(self.curr()) {
            (self, _) = self.advance();
        }

        self
    }
}
