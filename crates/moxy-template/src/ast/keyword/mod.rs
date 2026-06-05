mod tmpl_for;
mod tmpl_if;
mod tmpl_match;

use moxy_token::keyword::{For, If, Match};
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::At;
use moxy_token::{LexError, Parse, ToTokens, TokenStream};
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
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let at_punct = stream.parse::<At>()?;

        if let Some(if_kw) = stream.parse_if::<If>() {
            return Ok(Self::If(TmplIf::parse_after_keyword_if(stream, at_punct, if_kw)?));
        }

        if let Some(for_kw) = stream.parse_if::<For>() {
            return Ok(Self::For(TmplFor::parse_after_keyword_for(stream, at_punct, for_kw)?));
        }

        if let Some(match_kw) = stream.parse_if::<Match>() {
            return Ok(Self::Match(TmplMatch::parse_after_keyword_match(stream, at_punct, match_kw)?));
        }

        Err(LexError::new(stream.span())
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
