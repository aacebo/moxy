use moxy_token::{Group, TokenTree};

use crate::{Cursor, Parse, ParseError, Parser};

impl Parse for Group {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        next.is_group()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Group(v)) => {
                parser.advance();
                Ok(v.clone())
            }
            _ => Err(parser.expected("delimited group")),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}
