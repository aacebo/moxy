use moxy_token::{Delim, Span, ToTokens, TokenStream, TokenTree};

use crate::Parse;

/// Copyable transactional position within a token parser.
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

    pub fn peek<T: Parse>(self) -> bool {
        T::peek(self)
    }

    pub fn skip<T: Parse>(self) -> Option<Self> {
        T::skip(self)
    }

    pub fn is_delimited(self, delim: Delim) -> bool {
        match self.curr() {
            Some(TokenTree::Group(v)) => v.delim == delim,
            _ => false,
        }
    }

    pub fn descend(self, delim: Delim) -> Option<Self> {
        let (_, Some(TokenTree::Group(group))) = self.advance() else {
            return None;
        };

        if group.delim != delim {
            return None;
        }

        Some(Cursor::from_tokens(&group.tokens))
    }

    pub fn seek(mut self, i: usize) -> Self {
        self.index = i;
        self
    }

    pub fn offset(mut self, n: usize) -> Self {
        self.index += n;
        self
    }

    pub fn range(self, start: Self) -> &'a [TokenTree] {
        &self.tokens[start.index..self.index + 1]
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

impl<'a> ToTokens for Cursor<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let slice = &self.tokens[self.index..];
        tokens.extend(slice.to_vec());
    }
}
