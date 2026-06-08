use super::{ParseError, Peek};
use crate::span::{DelimSpan, fallback};
use crate::{Delim, LexError, Parse, Span, Token, TokenStream, TokenTree};

/// Split a span at `head_len` characters from its start, returning
/// `(head_span, rest_span)`. Only `Fallback` spans carry offsets we can split;
/// for compiler spans we reuse the whole span for both halves.
fn split_span(span: Span, head_len: usize) -> (Span, Span) {
    match span {
        Span::Fallback(s) => {
            let range = s.byte_range();
            let mid = (range.start + head_len) as u32;
            let head = fallback::Span::new(range.start as u32, mid);
            let rest = fallback::Span::new(mid, range.end as u32);
            (Span::Fallback(head), Span::Fallback(rest))
        }
        other => (other, other),
    }
}

pub struct ParseStream<'a> {
    input: &'a TokenStream,
    index: usize,
    /// The leftover half of a glued punct (`>>`, `>=`, ...) after `Gt`/`Lt`
    /// peeled off its first character. Acts as a virtual "current" token that is
    /// consumed before `index` advances again.
    pending: Option<TokenTree>,
}

impl<'a> ParseStream<'a> {
    pub fn new(input: &'a TokenStream) -> Self {
        Self {
            input,
            index: 0,
            pending: None,
        }
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
            pending: self.pending.clone(),
        }
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
        let index = self.index;
        let pending = self.pending.clone();
        let res = T::peek(self);
        self.index = index;
        self.pending = pending;
        res
    }

    pub fn parse<T: Parse>(&mut self) -> Result<T, ParseError> {
        let mut fork = self.fork();
        let value = T::parse(&mut fork)?;
        self.seek(&fork);
        Ok(value)
    }

    /// Parse `T` if it matches; leave the stream unchanged otherwise.
    pub fn parse_if<T: Parse>(&mut self) -> Option<T> {
        let mut fork = self.fork();
        let v = T::parse(&mut fork).ok()?;
        self.seek(&fork);
        Some(v)
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
            items.push(T::parse(self)?);
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
        use crate::Punctuation;

        let punct = match self.curr() {
            Some(TokenTree::Token(Token::Punct(p))) => *p,
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
        let remainder = Self::scan_punct(rest)?;
        let full = punct.span();
        let (head_span, rest_span) = split_span(full, head.len());

        let mut remainder = remainder;
        remainder.set_span(rest_span);

        self.advance();
        self.pending = Some(Punctuation::into_token_tree(remainder));
        Some(head_span)
    }

    /// Lex a single punctuation token from `text` (must be exactly one punct).
    fn scan_punct(text: &str) -> Option<crate::Punctuation> {
        use crate::Punctuation;
        use crate::lex::{Cursor, Scan};

        let cursor = Cursor::new(text, 0);
        <Punctuation as Scan>::scan(cursor).ok().map(|(_, op)| op)
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

#[cfg(test)]
mod tests {
    use crate::{Ident, Token, TokenStream, TokenTree};

    #[test]
    fn empty_stream() {
        let stream = TokenStream::new();
        let ps = stream.parse();
        assert!(ps.is_empty());
    }

    #[test]
    fn simple_idents_and_punct() {
        let stream = "a + b".parse::<TokenStream>().unwrap();
        let mut ps = stream.parse();

        assert!(matches!(ps.advance().unwrap(), TokenTree::Token(Token::Ident(_))));
        assert!(matches!(ps.advance().unwrap(), TokenTree::Token(Token::Punct(_))));
        assert!(matches!(ps.advance().unwrap(), TokenTree::Token(Token::Ident(_))));
        assert!(ps.is_empty());
    }

    #[test]
    fn peek_does_not_consume() {
        let stream = "a b".parse::<TokenStream>().unwrap();
        let mut ps = stream.parse();

        assert!(matches!(ps.peek::<Ident>(), true,));
        assert!(matches!(ps.peek::<Ident>(), true,));
        assert!(matches!(ps.parse::<Ident>(), Ok(_),));
        assert!(!ps.is_empty()); // "b" remains
    }

    #[test]
    fn fork_does_not_advance_original() {
        let stream = "a b".parse::<TokenStream>().unwrap();
        let mut ps = stream.parse();
        let mut fork = ps.fork();

        assert!(matches!(fork.parse::<Ident>(), Ok(_),)); // "a"
        assert!(matches!(ps.peek::<Ident>(), true,)); // still "a"
    }

    #[test]
    fn commit_fork() {
        let stream = "a b".parse::<TokenStream>().unwrap();
        let mut ps = stream.parse();
        let mut fork = ps.fork();

        fork.advance().unwrap(); // advance fork past "a"

        // original still at "a"
        assert!(matches!(ps.parse::<Ident>(), Ok(_),));

        // commit fork progress to original
        ps.seek(&fork);
        assert!(matches!(ps.peek::<Ident>(), true,)); // now at "b"
    }

    #[test]
    fn group_token_accessible() {
        let stream = "(a + b) c".parse::<TokenStream>().unwrap();
        let mut ps = stream.parse();
        let group = ps.advance().unwrap();
        assert!(matches!(group, TokenTree::Group(_)));

        if let TokenTree::Group(g) = group {
            let tokens = g.stream();
            let mut inner = tokens.parse();
            debug_assert!(matches!(inner.advance().unwrap(), TokenTree::Token(Token::Ident(_)))); // "a"
        }
    }
}
