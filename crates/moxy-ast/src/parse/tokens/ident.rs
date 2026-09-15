use moxy_token::{Ident, TokenTree};

use crate::{Cursor, Parse, ParseError, Parser};

impl Parse for Ident {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        next.is_ident()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.advance().cloned() {
            Some(TokenTree::Ident(v)) => Ok(v),
            Some(other) => Err(parser.error(format!("expected ident, received \"{}\"", other))),
            None => Err(parser.error("expected ident, received \"<EOF>\"")),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor.offset(1).into()
    }
}
