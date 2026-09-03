use crate::{Parse, ParseError, Parser};
use moxy_token::Token;
use moxy_token::{Delim, Group, LexError, Span, Spanner, ToTokens, TokenStream, TokenTree};

use crate::Path;

/// A macro invocation (`path!(...)`, `path![...]`, `path!{...}`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct MacroCall {
    pub path: Path,
    pub bang: Token![!],
    pub body: Group,
}

impl MacroCall {
    pub fn parse_semi(parser: &Parser) -> Result<(Self, Option<Token![;]>), ParseError> {
        let mac = parser.parse::<Self>()?;
        let semi = parser.parse_if::<Token![;]>();
        Ok((mac, semi))
    }

    /// The delimiter of the macro body (`(`, `[`, or `{`).
    pub fn delim(&self) -> Delim {
        self.body.delim()
    }

    /// The token parser inside the macro body delimiters.
    pub fn tokens(&self) -> TokenStream {
        self.body.stream()
    }
}

impl Parse for MacroCall {
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let path = parser.parse::<Path>()?;
        let bang = parser.parse::<Token![!]>()?;
        let body = match parser.curr() {
            Some(TokenTree::Group(g)) => {
                let g = g.clone();
                parser.advance();
                g
            }
            _ => {
                return Err(LexError::new(parser.span()).message("expected macro delimiter").into());
            }
        };

        Ok(Self { path, bang, body })
    }
}

impl Spanner for MacroCall {
    fn span(&self) -> Span {
        self.path.span().join(self.body.span().into())
    }
}

impl ToTokens for MacroCall {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.path.to_tokens(tokens);
        self.bang.to_tokens(tokens);
        tokens.extend_one(TokenTree::Group(self.body.clone()));
    }
}
