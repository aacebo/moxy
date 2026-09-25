use std::cell::Cell;

use moxy_token::span::DelimSpan;
use moxy_token::{Delim, Ident, Span, ToTokens, TokenStream, TokenTree};

use crate::parse::{Cursor, Parse, ParseError};

/// An AST representation of Rust parser syntax.
#[derive(Clone)]
pub struct Parser<'a> {
    cursor: Cell<Cursor<'a>>,
}

impl<'a> Parser<'a> {
    pub fn from_tokens(tokens: &'a TokenStream) -> Self {
        Self {
            cursor: Cell::new(Cursor::from_tokens(tokens)),
        }
    }

    pub fn from_cursor(cursor: Cursor<'a>) -> Self {
        Self {
            cursor: Cell::new(cursor),
        }
    }

    pub fn cursor(&self) -> Cursor<'a> {
        self.cursor.get()
    }

    pub fn is_empty(&self) -> bool {
        self.cursor.get().is_empty()
    }

    pub fn remaining(&self) -> usize {
        self.cursor.get().remaining()
    }

    pub fn span(&self) -> Span {
        self.cursor.get().span()
    }

    pub fn error(&self, message: impl std::fmt::Display) -> ParseError {
        ParseError::new(self.span(), message)
    }

    pub fn fork(&self) -> Self {
        Self {
            cursor: self.cursor.clone(),
        }
    }

    pub fn seek(&self, other: &Self) {
        self.cursor.set(other.cursor.get());
    }

    pub fn curr(&self) -> Option<&'a TokenTree> {
        self.cursor.get().curr()
    }

    pub fn next(&self) -> Option<&'a TokenTree> {
        self.cursor.get().next()
    }

    pub fn nth(&self, n: usize) -> Option<&'a TokenTree> {
        self.cursor.get().nth(n)
    }

    pub fn prev(&self) -> Option<&'a TokenTree> {
        self.cursor.get().prev()
    }

    pub fn offset(&self, n: usize) -> &Self {
        let next = self.cursor.get().offset(n);
        self.cursor.set(next);
        self
    }

    pub fn is_delimited(&self, delim: Delim) -> bool {
        self.cursor.get().is_delimited(delim)
    }

    pub fn descend<P: FnOnce(&Parser) -> bool + 'static>(&self, delim: Delim, p: P) -> Result<bool, ParseError> {
        let parser = self.parse_group(delim)?;
        Ok(p(&parser))
    }
}

impl<'a> Parser<'a> {
    pub fn advance(&self) -> Option<&'a TokenTree> {
        let (cursor, token) = self.cursor.get().advance();
        self.cursor.set(cursor);
        token
    }

    pub fn advance_by(&self, n: usize) -> Option<&'a [TokenTree]> {
        let original = self.cursor.get();
        let (cursor, tokens) = original.advance_by(n);

        if tokens.is_some() {
            self.cursor.set(cursor);
        }

        tokens
    }

    pub fn skip_until<F: Fn(Option<&TokenTree>) -> bool>(&self, pred: F) -> &Self {
        self.cursor.set(self.cursor.get().skip_until(pred));
        self
    }
}

impl Parser<'_> {
    pub fn parse_while<T: Parse>(&self) -> Vec<T> {
        let mut items = Vec::new();

        while T::peek(self.cursor()) {
            let fork = self.fork();

            match T::parse(&fork) {
                Ok(item) => {
                    self.seek(&fork);
                    items.push(item);
                }
                Err(_) => break,
            }
        }

        items
    }

    pub fn parse_until_empty<T: Parse>(&self) -> Result<Vec<T>, ParseError> {
        let mut items = Vec::new();

        while !self.is_empty() {
            items.push(T::parse(self)?);
        }

        Ok(items)
    }
}

impl<'a> Parser<'a> {
    pub fn parse_group(&self, delim: Delim) -> Result<Parser<'a>, ParseError> {
        match self.cursor().curr() {
            Some(TokenTree::Group(group)) if group.delim == delim => {
                self.advance();
                Ok(Self::from_tokens(&group.tokens))
            }
            _ => Err(self.error(format!("expected `{}` delimiter", delim.as_str()))),
        }
    }

    pub fn parse_group_spanned(&self, delim: Delim) -> Result<(DelimSpan, Parser<'a>), ParseError> {
        match self.cursor().curr() {
            Some(TokenTree::Group(group)) if group.delim == delim => {
                self.advance();
                Ok((group.span, Self::from_tokens(&group.tokens)))
            }
            _ => Err(self.error(format!("expected `{}` delimiter", delim.as_str()))),
        }
    }

    pub fn parse_ident_any(&self) -> Result<Ident, ParseError> {
        match self.advance() {
            Some(TokenTree::Ident(ident)) => Ok(ident.clone()),
            Some(TokenTree::Keyword(keyword)) => Ok(Ident::new(keyword.as_str()).with_span(keyword.span())),
            Some(other) => Err(self.error(format!("expected Ident, received \"{}\"", other))),
            None => Err(self.error("expected Ident, received \"<EOF>\"")),
        }
    }
}

impl<'a> ToTokens for Parser<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.cursor.get().to_tokens(tokens);
    }
}
