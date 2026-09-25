mod tmpl_for;
mod tmpl_if;
mod tmpl_match;

use moxy_ast::{Cursor, Parse, ParseError, Parser};
use moxy_token::{ToTokens, TokenStream};
pub use tmpl_for::*;
pub use tmpl_if::*;
pub use tmpl_match::*;

#[doc = "A template `@`-directive: `@if`, `@for`, or `@match`."]
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug))]
pub enum TmplKeyword {
    If(TmplIf),
    For(TmplFor),
    Match(TmplMatch),
}

impl Parse for TmplKeyword {
    fn peek(cursor: Cursor<'_>) -> bool {
        TmplIf::peek(cursor) || TmplFor::peek(cursor) || TmplMatch::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        if TmplIf::peek(parser.cursor()) {
            return Ok(Self::If(parser.parse()?));
        }

        if TmplFor::peek(parser.cursor()) {
            return Ok(Self::For(parser.parse()?));
        }

        if TmplMatch::peek(parser.cursor()) {
            return Ok(Self::Match(parser.parse()?));
        }

        parser.error("expected template directive").into()
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        if TmplIf::peek(cursor) {
            return TmplIf::skip(cursor);
        }

        if TmplFor::peek(cursor) {
            return TmplFor::skip(cursor);
        }

        if TmplMatch::peek(cursor) {
            return TmplMatch::skip(cursor);
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
