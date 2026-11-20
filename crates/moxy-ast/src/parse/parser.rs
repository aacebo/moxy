use std::cell::Cell;

use moxy_token::{Delim, DelimSpan, Ident, Span, ToTokens, TokenStream, TokenTree};

use crate::{Cursor, Parse, ParseError};

/// An AST representation of Rust parser syntax.
#[derive(Clone)]
pub struct Parser<'a> {
    cursor: Cell<Cursor<'a>>,
    // The delimiter of the group the parser reads and the span of its closing
    // delimiter, which an error at the end of its tokens names and points at
    group: Option<(Delim, Span)>,
}

impl<'a> Parser<'a> {
    pub fn from_tokens(tokens: &'a TokenStream) -> Self {
        Self {
            cursor: Cell::new(Cursor::from_tokens(tokens)),
            group: None,
        }
    }

    pub fn from_cursor(cursor: Cursor<'a>) -> Self {
        Self {
            cursor: Cell::new(cursor),
            group: None,
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

    /// An error at the current token, which is not the `expected` input, that
    /// names the token or the end of the input, as "expected `;`, found `}`".
    pub fn expected(&self, expected: impl std::fmt::Display) -> ParseError {
        ParseError::new(self.found_span(), format!("expected {expected}, found {}", self.found()))
    }

    /// An error at the current token, which no input expects, as "unexpected
    /// `@`".
    pub fn unexpected(&self) -> ParseError {
        ParseError::new(self.found_span(), format!("unexpected {}", self.found()))
    }

    // The current token as a message quotes it: at the end of a group's
    // tokens, the delimiter that closes it, or else the end of the input
    fn found(&self) -> String {
        match (self.curr(), self.group) {
            (Some(token), _) => found_token(token),
            (None, Some((Delim::None, _))) => "end of group".to_owned(),
            (None, Some((delim, _))) => quoted(&delim.close().to_string()),
            (None, None) => "end of input".to_owned(),
        }
    }

    // Where the current token is, or at the end of a delimited group's tokens
    // the delimiter that closes it
    fn found_span(&self) -> Span {
        match (self.curr(), self.group) {
            (None, Some((delim, close))) if delim != Delim::None => close,
            _ => self.span(),
        }
    }

    /// A parser over the tokens of `group`, which names and points at the
    /// group's closing delimiter in an error at the end of its tokens.
    pub fn from_group(group: &'a moxy_token::Group) -> Self {
        Self {
            cursor: Cell::new(Cursor::from_tokens(&group.tokens)),
            group: Some((group.delim, group.span.close())),
        }
    }

    pub fn fork(&self) -> Self {
        Self {
            cursor: self.cursor.clone(),
            group: self.group,
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

    /// Returns an error at the first remaining token unless the input is fully
    /// consumed, so a list that stops early inside a delimited group fails
    /// instead of dropping the rest of the group.
    pub fn ensure_empty(&self) -> Result<(), ParseError> {
        if self.is_empty() { Ok(()) } else { Err(self.unexpected()) }
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
                Ok(Self::from_group(group))
            }
            _ => Err(self.expected(delim_name(delim))),
        }
    }

    pub fn parse_group_spanned(&self, delim: Delim) -> Result<(DelimSpan, Parser<'a>), ParseError> {
        match self.cursor().curr() {
            Some(TokenTree::Group(group)) if group.delim == delim => {
                self.advance();
                Ok((group.span, Self::from_group(group)))
            }
            _ => Err(self.expected(delim_name(delim))),
        }
    }

    pub fn parse_ident_any(&self) -> Result<Ident, ParseError> {
        let ident = match self.curr() {
            Some(TokenTree::Ident(ident)) => ident.clone(),
            Some(TokenTree::Keyword(keyword)) => Ident::new(keyword.as_str()).with_span(keyword.span()),
            _ => return Err(self.expected("identifier")),
        };

        self.advance();
        Ok(ident)
    }
}

// A token as an error message quotes it: a group as its delimiters alone, as
// `{ ... }`, so a message does not print a whole macro body
pub(crate) fn received(token: &TokenTree) -> String {
    match token {
        // A group without delimiters, which only a stream from the compiler
        // holds, by its first token
        TokenTree::Group(group) if group.delim == Delim::None => match group.tokens.first() {
            Some(first) if group.tokens.len() > 1 => format!("{} ...", received(first)),
            Some(first) => received(first),
            None => String::new(),
        },
        TokenTree::Group(group) if group.tokens.is_empty() => format!("{}{}", group.delim.open(), group.delim.close()),
        TokenTree::Group(group) => format!("{} ... {}", group.delim.open(), group.delim.close()),
        other => other.to_string(),
    }
}

// A node a message names as found, by its first token, so the message stays
// short and spaced as written
pub(crate) fn found_node(node: &impl moxy_token::ToTokenStream) -> String {
    found_tokens(&node.to_token_stream())
}

// Tokens a message names as found: whole when they are a short path, as
// `a::b`, or else by their first token
pub(crate) fn found_tokens(tokens: &TokenStream) -> String {
    found_preview(tokens, false)
}

pub(crate) fn found_value(tokens: &TokenStream) -> String {
    found_preview(tokens, true)
}

fn found_preview(tokens: &TokenStream, include_expression: bool) -> String {
    if (2..=7).contains(&tokens.len()) {
        let path = crate::Path::skip_mod_style(Cursor::from_tokens(tokens)).is_some_and(Cursor::is_empty);

        if (path || include_expression)
            && let Some(preview) = moxy_token::__private::short_token_text(tokens)
        {
            return quoted(&if path { compact(tokens) } else { preview });
        }
    }

    match tokens.first() {
        Some(token) => found_token(token),
        None => "end of input".to_owned(),
    }
}

// A token as a message names it, an empty group without delimiters included
fn found_token(token: &TokenTree) -> String {
    match received(token) {
        text if text.is_empty() => "empty group".to_owned(),
        text => quoted(&text),
    }
}

// The text of a path's tokens, with no space around `::`
fn compact(tokens: &TokenStream) -> String {
    let mut text = String::new();
    let mut word = false;

    for token in tokens.iter() {
        let is_word = !matches!(token, TokenTree::Punct(_));

        if word && is_word {
            text.push(' ');
        }

        text.push_str(&token.to_string());
        word = is_word;
    }

    text
}

// Code in a message in backticks, or in double ones with spaces when it holds
// a backtick
pub(crate) fn quoted(code: &str) -> String {
    moxy_token::__private::quoted(code)
}

impl<'a> ToTokens for Parser<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.cursor.get().to_tokens(tokens);
    }
}

// How a message names a delimited group that it expects
fn delim_name(delim: Delim) -> String {
    match delim {
        Delim::None => "group without delimiters".to_owned(),
        delim => format!("`{}`", delim.open()),
    }
}
