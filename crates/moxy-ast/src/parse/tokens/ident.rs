use moxy_token::{Ident, TokenTree};

use crate::{Cursor, Parse, ParseError, Parser};

impl Parse for Ident {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        next.is_ident() || is_weak_keyword(next)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.advance().cloned() {
            Some(TokenTree::Ident(v)) => Ok(v),
            // A weak keyword is an identifier wherever its keyword use does not apply, as in
            // `fn default()` or `self.raw`
            Some(TokenTree::Keyword(keyword)) if is_weak_keyword(&TokenTree::Keyword(keyword)) => {
                Ok(Ident::new(keyword.as_str()).with_span(keyword.span()))
            }
            Some(other) => Err(parser.error(format!("expected ident, received \"{}\"", other))),
            None => Err(parser.error("expected ident, received \"<EOF>\"")),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

// The keywords that are keywords only in some positions and identifiers elsewhere
pub(crate) fn is_weak_keyword(token: &TokenTree) -> bool {
    token.is_keyword_auto()
        || token.is_keyword_default()
        || token.is_keyword_macro_rules()
        || token.is_keyword_raw()
        || token.is_keyword_safe()
        || token.is_keyword_union()
}
