use moxy_token::Token;
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::span::Spanner;
use moxy_token::{Parse, Span, ToTokens, TokenStream};

use super::Type;
use crate::{Delimited, Punctuated};

/// A tuple type (e.g. `()`, `(A, B)`, `(T,)`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TypeTuple {
    pub elems: Delimited<Punctuated<Type, Token![,]>>,
}

impl Parse for TypeTuple {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let elems = Delimited::parse_paren_with(stream, Punctuated::parse_terminated)?;
        Ok(Self { elems })
    }
}

impl Spanner for TypeTuple {
    fn span(&self) -> Span {
        self.elems.span()
    }
}

impl ToTokens for TypeTuple {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.elems.to_tokens(tokens);
    }
}
