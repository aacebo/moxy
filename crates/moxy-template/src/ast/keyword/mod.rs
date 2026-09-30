mod tmpl_for;
mod tmpl_if;
mod tmpl_match;

pub use tmpl_for::*;
pub use tmpl_if::*;
pub use tmpl_match::*;

use moxy_ast::{Cursor, Expr, Parse, ParseError, Parser};
use moxy_token::{Delim, ToTokens, TokenStream, TokenTree};

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
            return Ok(Self::If(<_ as Parse>::parse(parser)?));
        }

        if TmplFor::peek(parser.cursor()) {
            return Ok(Self::For(<_ as Parse>::parse(parser)?));
        }

        if TmplMatch::peek(parser.cursor()) {
            return Ok(Self::Match(<_ as Parse>::parse(parser)?));
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

pub(super) fn parse_unparenthesized_expr(parser: &Parser) -> Result<TokenStream, ParseError> {
    let mut expr = TokenStream::new();

    loop {
        if matches!(parser.curr(), Some(TokenTree::Group(group)) if group.delim == Delim::Brace) {
            let expr_parser = Parser::from_tokens(&expr);

            if <Expr as Parse>::parse(&expr_parser).is_ok() && expr_parser.is_empty() {
                return Ok(expr);
            }
        }

        let Some(token) = parser.advance() else {
            return parser.error("expected template directive body").into();
        };

        expr.extend_one(token.clone());
    }
}

pub(super) fn skip_unparenthesized_expr(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
    let mut expr = TokenStream::new();

    loop {
        if matches!(cursor.curr(), Some(TokenTree::Group(group)) if group.delim == Delim::Brace) {
            let expr_parser = Parser::from_tokens(&expr);

            if <Expr as Parse>::parse(&expr_parser).is_ok() && expr_parser.is_empty() {
                return Some(cursor);
            }
        }

        expr.extend_one(cursor.curr()?.clone());
        cursor = cursor.offset(1);
    }
}
