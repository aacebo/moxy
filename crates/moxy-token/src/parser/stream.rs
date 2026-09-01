use super::{ParseError, Peek};
use crate::span::DelimSpan;
use crate::{Delim, LexError, Parse, Span, TokenStream, TokenTree};

pub struct ParseStream<'a> {
    input: &'a TokenStream,
    index: usize,
    depth: usize,
    config: ParseConfig,
    pending: Option<TokenTree>,
}

impl<'a> ParseStream<'a> {
    pub fn new(input: &'a TokenStream) -> Self {
        Self {
            input,
            index: 0,
            depth: 0,
            config: Default::default(),
            pending: None,
        }
    }

    pub fn from_config(input: &'a TokenStream, config: ParseConfig) -> Self {
        Self {
            input,
            index: 0,
            depth: 0,
            config,
            pending: None,
        }
    }

    pub fn config(&self) -> &ParseConfig {
        &self.config
    }

    pub fn is_empty(&self) -> bool {
        self.pending.is_none() && self.index >= self.input.len()
    }

    pub fn span(&self) -> Span {
        if let Some(t) = &self.pending {
            return t.span();
        }

        self.input.get(self.index).map(|t| t.span()).unwrap_or_default()
    }

    pub fn fork(&self) -> Self {
        Self {
            input: self.input,
            index: self.index,
            depth: self.depth + 1,
            config: self.config,
            pending: self.pending.clone(),
        }
    }

    /// Create a non-consuming fork whose parser activity is omitted from traces.
    ///
    /// This is intended for grammar lookahead that needs more information than
    /// [`peek`](Self::peek)'s boolean result.
    pub fn lookahead(&self) -> Self {
        let mut fork = self.fork();
        fork.config.trace = false;
        fork
    }

    pub fn seek(&mut self, other: &Self) {
        self.index = other.index;
        self.pending = other.pending.clone();
    }
}

impl<'a> ParseStream<'a> {
    pub fn remaining(&self) -> usize {
        self.input.len().saturating_sub(self.index)
    }

    pub fn curr(&self) -> Option<&TokenTree> {
        if let Some(t) = &self.pending {
            return Some(t);
        }

        self.input.get(self.index)
    }

    /// Look ahead `n` tokens without consuming (`nth(0)` == `curr`). When a glued
    /// punct has been split, the pending half is `nth(0)` and the real stream
    /// follows it.
    pub fn nth(&self, n: usize) -> Option<&TokenTree> {
        if let Some(t) = &self.pending {
            return if n == 0 { Some(t) } else { self.input.get(self.index + n - 1) };
        }

        self.input.get(self.index + n)
    }

    pub fn prev(&self) -> Option<&TokenTree> {
        self.input.get(self.index - 1)
    }

    pub fn peek<T: Peek>(&mut self) -> bool {
        T::peek(&mut self.lookahead())
    }

    pub fn parse<T: Parse>(&mut self) -> Result<T, ParseError> {
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

        let mut fork = self.fork();
        let value = match T::parse(&mut fork) {
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

        self.seek(&fork);
        Ok(value)
    }

    /// Parse `T` if it matches; leave the stream unchanged otherwise.
    pub fn parse_if<T: Parse>(&mut self) -> Option<T> {
        if self.peek::<T>() { self.parse().ok() } else { None }
    }

    /// Parse `T` repeatedly while it matches, collecting results. Never errors.
    pub fn parse_while<T: Parse>(&mut self) -> Vec<T> {
        let mut items = Vec::new();

        while let Some(item) = self.parse_if::<T>() {
            items.push(item);
        }

        items
    }

    /// Parse `T` until the stream is empty, propagating the first error.
    pub fn parse_until_empty<T: Parse>(&mut self) -> Result<Vec<T>, ParseError> {
        let mut items = Vec::new();

        while !self.is_empty() {
            items.push(self.parse()?);
        }

        Ok(items)
    }

    /// Discard leading `T` tokens while they match. Returns `self` for chaining.
    pub fn skip_while<T: Parse>(&mut self) -> &mut Self {
        while self.parse_if::<T>().is_some() {}
        self
    }

    /// Discard one `T` if it matches. Returns `self` for chaining.
    pub fn skip_if<T: Parse>(&mut self) -> &mut Self {
        self.parse_if::<T>();
        self
    }

    /// Advance past tokens until `pred(curr())` is true (does NOT consume the matching token).
    /// Returns `self` for chaining.
    pub fn skip_until<F: Fn(Option<&TokenTree>) -> bool>(&mut self, pred: F) -> &mut Self {
        while !self.is_empty() && !pred(self.curr()) {
            self.advance();
        }

        self
    }

    pub fn advance_by(&mut self, n: usize) -> Option<&[TokenTree]> {
        if self.index + n > self.input.len() {
            return None;
        }

        let start = self.index;
        self.index += n;
        Some(&self.input[start..self.index])
    }

    /// move the iterator forward and return the token. Consumes a pending split
    /// half first if one is present.
    pub fn advance(&mut self) -> Option<&TokenTree> {
        if self.pending.is_some() {
            self.pending = None;
            // The split half has been consumed; report the real token it came from.
            return self.input.get(self.index - 1);
        }

        self.advance_by(1)?.first()
    }

    /// Consume the next token as the single-char punct spelled `head`. If the
    /// next token is a longer glued punct that starts with `head` (e.g. `>>`,
    /// `>=`, `>>=`), peel off the first char and leave the remainder as a pending
    /// split token. Returns the span for the consumed head punct, or `None` if
    /// the next token isn't a punct starting with `head`.
    pub fn eat_punct_head(&mut self, head: &str) -> Option<Span> {
        let punct = match self.curr() {
            Some(TokenTree::Punct(p)) => *p,
            _ => return None,
        };

        let text = punct.as_str();

        if text == head {
            let span = punct.span();
            self.advance();
            return Some(span);
        }

        if !text.starts_with(head) {
            return None;
        }

        let rest = &text[head.len()..];
        let mut remainder = Self::scan_punct(rest)?;
        let full = punct.span();
        let (head_span, rest_span) = full.split(head.len());
        remainder.set_span(rest_span);

        self.advance();
        self.pending = Some(TokenTree::Punct(remainder));
        Some(head_span)
    }

    /// Lex a single punctuation token from `text` (must be exactly one punct).
    fn scan_punct(text: &str) -> Option<crate::Punctuation> {
        use crate::Punctuation;
        use crate::lex::{Cursor, Scan};

        let cursor = Cursor::new(text, 0);
        Punctuation::scan(cursor).ok().map(|(_, op)| op)
    }

    /// Consume a group with the given delimiter and return its inner token stream.
    /// The caller can then create a new ParseStream over the returned stream.
    pub fn parse_group(&mut self, delim: Delim) -> Result<TokenStream, ParseError> {
        let at = self.span();

        match self.curr() {
            Some(TokenTree::Group(g)) if g.delim() == delim => {
                let stream = g.stream();
                self.advance();
                Ok(stream)
            }
            _ => Err(LexError::new(at)
                .message(format!("expected `{}` delimiter", delim.as_str()))
                .into()),
        }
    }

    /// Like [`parse_group`](Self::parse_group), but also returns the group's
    /// `DelimSpan` (the open/close spans of its delimiters).
    pub fn parse_group_spanned(&mut self, delim: Delim) -> Result<(DelimSpan, TokenStream), ParseError> {
        let at = self.span();

        match self.curr() {
            Some(TokenTree::Group(g)) if g.delim() == delim => {
                let span = g.span();
                let stream = g.stream();
                self.advance();
                Ok((span, stream))
            }
            _ => Err(LexError::new(at)
                .message(format!("expected `{}` delimiter", delim.as_str()))
                .into()),
        }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ParseConfig {
    pub trace: bool,
}

enum Ansi {
    Blue,
    Green,
    Red,
    Reset,
}

impl std::fmt::Display for Ansi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Blue => write!(f, "\x1b[34m"),
            Self::Green => write!(f, "\x1b[32m"),
            Self::Red => write!(f, "\x1b[31m"),
            Self::Reset => write!(f, "\x1b[0m"),
        }
    }
}
