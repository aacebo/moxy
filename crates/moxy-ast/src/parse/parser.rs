use std::cell::Cell;

use moxy_token::span::DelimSpan;
use moxy_token::{Delim, Span, TokenStream, TokenTree};

use crate::parse::{Ansi, Cursor, Parse, ParseConfig, ParseError, Peek};

#[derive(Clone)]
pub struct Parser<'a> {
    cursor: Cell<Cursor<'a>>,
    config: ParseConfig,
    depth: usize,
}

impl<'a> Parser<'a> {
    pub fn from_tokens(tokens: &'a TokenStream) -> Self {
        Self::from_config(tokens, ParseConfig::default())
    }

    pub fn from_config(tokens: &'a TokenStream, config: ParseConfig) -> Self {
        Self {
            cursor: Cell::new(Cursor::from_tokens(tokens)),
            config,
            depth: 0,
        }
    }

    pub fn traceable(mut self) -> Self {
        self.config.trace = true;
        self
    }

    pub fn config(&self) -> &ParseConfig {
        &self.config
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
            config: self.config,
            depth: self.depth + 1,
        }
    }

    pub fn lookahead(&self) -> Self {
        let mut fork = self.fork();
        fork.config.trace = false;
        fork
    }

    pub fn seek(&self, other: &Self) {
        self.cursor.set(other.cursor.get());
    }
}

impl<'a> Parser<'a> {
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
    pub fn peek<T: Peek>(&self) -> bool {
        T::peek(&self.lookahead())
    }

    pub fn parse<T: Parse>(&self) -> Result<T, ParseError> {
        let name = std::any::type_name::<T>();

        if self.config.trace {
            println!(
                "{}{}-> {} @ ln {}, col {}{}",
                " ".repeat(self.depth),
                Ansi::Blue,
                name,
                self.span().start().line(),
                self.span().start().column(),
                Ansi::Reset,
            );
        }

        let fork = self.fork();
        let value = T::parse(&fork);

        if self.config.trace {
            let (color, span) = if value.is_ok() {
                (Ansi::Green, fork.span())
            } else {
                (Ansi::Red, self.span())
            };

            println!(
                "{}{}<- {} @ ln {}, col {}{}",
                " ".repeat(self.depth),
                color,
                name,
                span.end().line(),
                span.end().column(),
                Ansi::Reset,
            );
        }

        let value = value?;
        self.seek(&fork);
        Ok(value)
    }

    pub fn parse_if<T: Parse>(&self) -> Option<T> {
        self.parse().ok()
    }

    pub fn parse_while<T: Parse>(&self) -> Vec<T> {
        let mut items = Vec::new();

        while let Some(item) = self.parse_if::<T>() {
            items.push(item);
        }

        items
    }

    pub fn parse_until_empty<T: Parse>(&self) -> Result<Vec<T>, ParseError> {
        let mut items = Vec::new();

        while !self.is_empty() {
            items.push(self.parse()?);
        }

        Ok(items)
    }

    pub fn skip_while<T: Parse>(&self) -> &Self {
        while self.parse_if::<T>().is_some() {}
        self
    }

    pub fn skip_if<T: Parse>(&self) -> &Self {
        self.parse_if::<T>();
        self
    }
}

impl Parser<'_> {
    pub fn parse_group(&self, delim: Delim) -> Result<TokenStream, ParseError> {
        match self.curr() {
            Some(TokenTree::Group(group)) if group.delim() == delim => {
                self.advance();
                Ok(group.stream())
            }
            _ => Err(self.error(format!("expected `{}` delimiter", delim.as_str()))),
        }
    }

    pub fn parse_group_spanned(&self, delim: Delim) -> Result<(DelimSpan, TokenStream), ParseError> {
        match self.curr() {
            Some(TokenTree::Group(group)) if group.delim() == delim => {
                self.advance();
                Ok((group.span(), group.stream()))
            }
            _ => Err(self.error(format!("expected `{}` delimiter", delim.as_str()))),
        }
    }

    pub fn parse_ident_any(&self) -> Result<moxy_token::Ident, ParseError> {
        match self.advance() {
            Some(TokenTree::Ident(ident)) => Ok(ident.clone()),
            Some(TokenTree::Keyword(keyword)) => Ok(moxy_token::Ident::new(keyword.as_str()).with_span(keyword.span())),
            Some(other) => Err(self.error(format!("expected Ident, received \"{}\"", other))),
            None => Err(self.error("expected Ident, received \"<EOF>\"")),
        }
    }
}
