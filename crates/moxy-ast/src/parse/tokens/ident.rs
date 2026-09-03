use moxy_token::{Ident, TokenTree};

use crate::{Parse, ParseError, Parser, Peek};

impl Peek for Ident {
    fn peek(parser: &Parser) -> bool {
        let Some(next) = parser.curr() else {
            return false;
        };

        next.is_ident()
    }
}

impl Parse for Ident {
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.advance().cloned() {
            Some(TokenTree::Ident(v)) => Ok(v),
            Some(other) => Err(parser.error(format!("expected ident, received \"{}\"", other))),
            None => Err(parser.error("expected ident, received \"<EOF>\"")),
        }
    }
}
