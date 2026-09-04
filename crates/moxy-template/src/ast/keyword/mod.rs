mod tmpl_for;
mod tmpl_if;
mod tmpl_match;

use moxy_ast::{Parse, ParseError, Parser, Token};
use moxy_token::{LexError, ToTokens, TokenStream};
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
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let at_punct = parser.parse::<Token![@]>()?;

        if let Some(if_kw) = parser.parse_if::<Token![if]>() {
            return Ok(Self::If(TmplIf::parse_after_keyword_if(parser, at_punct, if_kw)?));
        }

        if let Some(for_kw) = parser.parse_if::<Token![for]>() {
            return Ok(Self::For(TmplFor::parse_after_keyword_for(parser, at_punct, for_kw)?));
        }

        if let Some(match_kw) = parser.parse_if::<Token![match]>() {
            return Ok(Self::Match(TmplMatch::parse_after_keyword_match(parser, at_punct, match_kw)?));
        }

        Err(LexError::new(parser.span())
            .message("expected `if`, `for`, or `match` after `@`")
            .into())
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
