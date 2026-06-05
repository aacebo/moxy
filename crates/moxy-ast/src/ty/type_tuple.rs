use moxy_token::parse::{ParseError, ParseStream};
use moxy_token::punct::Comma;
use moxy_token::span::Spanner;
use moxy_token::{Parse, Span, ToTokens, TokenStream};

use super::Type;
use crate::{Delimited, Punctuated};

#[doc = "A tuple type (e.g. `()`, `(A, B)`, `(T,)`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TypeTuple {
    pub span: Span,
    pub paren: Delimited<Punctuated<Type, Comma>>,
}

impl Parse for TypeTuple {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let paren = Delimited::parse_paren_with(stream, Punctuated::parse_terminated)?;
        Ok(Self {
            span: paren.span(),
            paren,
        })
    }
}

impl ToTokens for TypeTuple {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.paren.to_tokens(tokens);
    }
}
