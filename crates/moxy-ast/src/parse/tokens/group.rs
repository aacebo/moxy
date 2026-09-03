use moxy_token::{Group, TokenTree};

use crate::{Parse, ParseError, Parser, Peek};

impl Peek for Group {
    fn peek(parser: &Parser) -> bool {
        let Some(next) = parser.next() else {
            return false;
        };

        next.is_group()
    }
}

impl Parse for Group {
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.advance() {
            Some(TokenTree::Group(v)) => Ok(v.clone()),
            _ => parser.error("expected group").into(),
        }
    }
}
