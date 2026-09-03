mod expr;

use std::cell::Cell;

use moxy_token::{Delim, Span, TokenStream, TokenTree, span::DelimSpan};

use crate::parse::{Ansi, Cursor, Parse, ParseConfig, ParseError, Peek};

#[derive(Clone)]
pub struct Parser<'a> {
    cursor: Cell<Cursor<'a>>,
    config: ParseConfig,
    depth: usize,
}

impl<'a> Parser<'a> {
    pub fn from_tokens(tokens: &'a TokenStream) -> Self {
        Self {
            cursor: Cursor::from_tokens(tokens).into(),
            config: Default::default(),
            depth: 0,
        }
    }

    pub fn traceable(mut self) -> Self {
        self.config.trace = true;
        self
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
}

impl<'a> Parser<'a> {
    pub fn next(&self) -> Option<&TokenTree> {
        self.cursor.get().next()
    }

    pub fn curr(&self) -> Option<&TokenTree> {
        self.cursor.get().curr()
    }

    pub fn prev(&self) -> Option<&TokenTree> {
        self.cursor.get().prev()
    }

    pub fn advance(&self) -> Option<&TokenTree> {
        let cursor = self.cursor.get();
        let token = cursor.advance();
        self.cursor.set(cursor);
        token
    }

    pub fn advance_by(&self, n: usize) -> Option<&'a [TokenTree]> {
        let cursor = self.cursor.get();
        let tokens = cursor.advance_by(n);
        self.cursor.set(cursor);
        tokens
    }
}

impl<'a> Parser<'a> {
    pub fn peek<T: Peek>(&self) -> bool {
        T::peek(&self.fork())
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
        let value = match T::parse(&fork) {
            Err(err) => {
                if self.config.trace {
                    println!(
                        "{}{}<- {} @ ln {}, col {}{}",
                        " ".repeat(self.depth),
                        Ansi::Red,
                        name,
                        self.span().end().line(),
                        self.span().end().column(),
                        Ansi::Reset,
                    );
                }

                Err(err)
            }
            Ok(v) => {
                if self.config.trace {
                    println!(
                        "{}{}<- {} @ ln {}, col {}{}",
                        " ".repeat(self.depth),
                        Ansi::Green,
                        name,
                        self.span().end().line(),
                        self.span().end().column(),
                        Ansi::Reset,
                    );
                }

                Ok(v)
            }
        }?;

        self.cursor.set(fork.cursor.get());
        Ok(value)
    }

    /// Parse `T` if it matches; leave the stream unchanged otherwise.
    pub fn parse_if<T: Parse>(&self) -> Option<T> {
        self.parse().ok()
    }

    /// Parse `T` repeatedly while it matches, collecting results. Never errors.
    pub fn parse_while<T: Parse>(&self) -> Vec<T> {
        let mut items = Vec::new();

        while let Some(item) = self.parse_if::<T>() {
            items.push(item);
        }

        items
    }

    /// Parse `T` until the stream is empty, propagating the first error.
    pub fn parse_until_empty<T: Parse>(&self) -> Result<Vec<T>, ParseError> {
        let mut items = Vec::new();

        while !self.cursor.get().is_empty() {
            items.push(self.parse()?);
        }

        Ok(items)
    }

    /// Discard leading `T` tokens while they match. Returns `self` for chaining.
    pub fn skip_while<T: Parse>(&self) -> &Self {
        while self.parse_if::<T>().is_some() {}
        self
    }

    /// Discard one `T` if it matches. Returns `self` for chaining.
    pub fn skip_if<T: Parse>(&self) -> &Self {
        self.parse_if::<T>();
        self
    }
}

impl<'a> Parser<'a> {
    /// Consume a group with the given delimiter and return its inner token stream.
    /// The caller can then create a new Cursor over the returned stream.
    pub fn parse_group(&self, delim: Delim) -> Result<TokenStream, ParseError> {
        match self.cursor.get().curr() {
            Some(TokenTree::Group(g)) if g.delim() == delim => {
                let stream = g.stream();
                self.advance();
                Ok(stream)
            }
            _ => self.error(format!("expected `{}` delimiter", delim.as_str())).into(),
        }
    }

    /// Like [`parse_group`](Self::parse_group), but also returns the group's
    /// `DelimSpan` (the open/close spans of its delimiters).
    pub fn parse_group_spanned(&self, delim: Delim) -> Result<(DelimSpan, TokenStream), ParseError> {
        match self.cursor.get().curr() {
            Some(TokenTree::Group(g)) if g.delim() == delim => {
                let span = g.span();
                let stream = g.stream();
                self.advance();
                Ok((span, stream))
            }
            _ => self.error(format!("expected `{}` delimiter", delim.as_str())).into(),
        }
    }
}
