use super::{ParseError, Peek};
use crate::token::{Delim, LexError, Punctuation, Token};
use crate::{Parse, Span, TokenStream, TokenTree};

pub struct ParseStream<'a> {
    input: &'a TokenStream,
    index: usize,
    /// A "half-consumed" `>>`: after `eat_angle_close` splits a `Shr`, one `>`
    /// remains for the enclosing angle level, exposed here as a virtual `Gt`.
    pending_gt: Option<Span>,
}

impl<'a> ParseStream<'a> {
    pub fn new(input: &'a TokenStream) -> Self {
        Self {
            input,
            index: 0,
            pending_gt: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.pending_gt.is_none() && self.index >= self.input.len()
    }

    pub fn span(&self) -> Span {
        if let Some(span) = self.pending_gt {
            return span;
        }
        self.input.get(self.index).map(|t| t.span()).unwrap_or_default()
    }

    pub fn fork(&self) -> Self {
        Self {
            input: self.input,
            index: self.index,
            pending_gt: self.pending_gt,
        }
    }

    pub fn seek(&mut self, other: &Self) {
        self.index = other.index;
        self.pending_gt = other.pending_gt;
    }

    /// True if the next token closes an angle-bracket group: `>` (`Gt`) or the
    /// first `>` of a `>>` (`Shr`), or a pending split `>`.
    pub fn peek_angle_close(&mut self) -> bool {
        if self.pending_gt.is_some() {
            return true;
        }
        matches!(
            self.curr(),
            Some(TokenTree::Token(Token::Punct(Punctuation::Gt(_) | Punctuation::Shr(_))))
        )
    }

    /// Consume a single `>`, splitting a `>>` (`Shr`) so the second `>` remains
    /// available to the enclosing angle level.
    pub fn eat_angle_close(&mut self) -> Result<(), ParseError> {
        if self.pending_gt.take().is_some() {
            return Ok(());
        }
        let at = self.span();
        match self.curr() {
            Some(TokenTree::Token(Token::Punct(Punctuation::Gt(_)))) => {
                self.advance();
                Ok(())
            }
            Some(TokenTree::Token(Token::Punct(Punctuation::Shr(op)))) => {
                let span = op.span();
                self.advance();
                self.pending_gt = Some(span);
                Ok(())
            }
            _ => Err(LexError::new(at).message("expected `>`").into()),
        }
    }
}

impl<'a> ParseStream<'a> {
    pub fn remaining(&self) -> usize {
        self.input.len().saturating_sub(self.index)
    }

    pub fn curr(&self) -> Option<&TokenTree> {
        self.input.get(self.index)
    }

    /// Look ahead `n` tokens without consuming (`nth(0)` == `curr`). Ignores the
    /// `pending_gt` split state (callers using this aren't mid-angle-close).
    pub fn nth(&self, n: usize) -> Option<&TokenTree> {
        self.input.get(self.index + n)
    }

    pub fn prev(&self) -> Option<&TokenTree> {
        self.input.get(self.index - 1)
    }

    pub fn peek<T: Peek>(&mut self) -> Option<T> {
        let index = self.index;
        let res = T::peek(self);
        self.index = index;
        res
    }

    pub fn parse<T: Parse>(&mut self) -> Result<T, ParseError> {
        T::parse(self)
    }

    /// Greedily parse `T` until one stops matching (each attempt is fork-guarded).
    pub fn parse_vec<T: Parse>(&mut self) -> Result<Vec<T>, ParseError> {
        let mut items = Vec::new();

        loop {
            let mut fork = self.fork();

            match T::parse(&mut fork) {
                Ok(v) => {
                    self.seek(&fork);
                    items.push(v);
                }
                Err(_) => break,
            }
        }

        Ok(items)
    }

    /// Parse `T` only if it matches, leaving the stream unchanged otherwise.
    pub fn parse_opt<T: Parse>(&mut self) -> Option<T> {
        let mut fork = self.fork();

        match T::parse(&mut fork) {
            Ok(v) => {
                self.seek(&fork);
                Some(v)
            }
            Err(_) => None,
        }
    }

    pub fn advance_by(&mut self, n: usize) -> Option<&[TokenTree]> {
        if self.index + n > self.input.len() {
            return None;
        }

        let start = self.index;
        self.index += n;
        Some(&self.input[start..self.index])
    }

    /// move the iterator forward and return the token.
    pub fn advance(&mut self) -> Option<&TokenTree> {
        self.advance_by(1)?.first()
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
}

#[cfg(test)]
mod tests {
    use crate::token::Ident;
    use crate::{Token, TokenStream, TokenTree};

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

        assert!(matches!(ps.peek::<Ident>(), Some(_),));
        assert!(matches!(ps.peek::<Ident>(), Some(_),));
        assert!(matches!(ps.parse::<Ident>(), Ok(_)));
        assert!(!ps.is_empty()); // "b" remains
    }

    #[test]
    fn fork_does_not_advance_original() {
        let stream = "a b".parse::<TokenStream>().unwrap();
        let mut ps = stream.parse();
        let mut fork = ps.fork();

        assert!(matches!(fork.parse::<Ident>(), Ok(_),)); // "a"
        assert!(matches!(ps.peek::<Ident>(), Some(_),)); // still "a"
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
        assert!(matches!(ps.peek::<Ident>(), Some(_),)); // now at "b"
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
