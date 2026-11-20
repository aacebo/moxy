use moxy_token::{Group, TokenTree};

use crate::{Cursor, Parse, ParseError, Parser};

impl Parse for Group {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        next.is_group()
    }

    // A group parses unparsed, as a macro body or attribute arguments, and a
    // traversal walks its tokens as deep as they nest, so each group nested in
    // it takes a level
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Group(v)) => {
                moxy_token::__private::Depth::enter_tokens(&v.tokens).ok_or_else(|| ParseError::too_deep(parser.span()))?;
                parser.advance();
                Ok(v.clone())
            }
            _ => Err(parser.expected("delimited group")),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        let Some(TokenTree::Group(group)) = cursor.curr() else {
            return None;
        };

        moxy_token::__private::Depth::enter_tokens(&group.tokens)?;
        Some(cursor.offset(1))
    }
}
