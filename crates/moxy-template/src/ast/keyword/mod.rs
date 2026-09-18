mod tmpl_for;
mod tmpl_if;
mod tmpl_match;

use moxy_ast::{Cursor, Parse, ParseError, Parser};
use moxy_token::{ToTokens, TokenStream};
pub use tmpl_for::*;
pub use tmpl_if::*;
pub use tmpl_match::*;

#[doc = "A template `@`-directive: `@if`, `@for`, or `@match`."]
#[derive(Debug, Clone)]
pub enum TmplKeyword {
    If(TmplIf),
    For(TmplFor),
    Match(TmplMatch),
}

impl Parse for TmplKeyword {
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.peek::<TmplIf>() || cursor.peek::<TmplFor>() || cursor.peek::<TmplMatch>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        if parser.peek::<TmplIf>() {
            return Ok(Self::If(parser.parse()?));
        }

        if parser.peek::<TmplFor>() {
            return Ok(Self::For(parser.parse()?));
        }

        if parser.peek::<TmplMatch>() {
            return Ok(Self::Match(parser.parse()?));
        }

        parser.error("expected template directive").into()
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        if cursor.peek::<TmplIf>() {
            return cursor.skip::<TmplIf>();
        }

        if cursor.peek::<TmplFor>() {
            return cursor.skip::<TmplFor>();
        }

        if cursor.peek::<TmplMatch>() {
            return cursor.skip::<TmplMatch>();
        }

        None
    }
}

impl ToTokens for TmplKeyword {
    fn to_tokens(&self, out: &mut TokenStream) {
        match self {
            Self::If(v) => v.to_tokens(out),
            Self::For(v) => v.to_tokens(out),
            Self::Match(v) => v.to_tokens(out),
        }
    }
}
