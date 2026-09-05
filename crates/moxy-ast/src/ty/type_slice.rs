use crate::{Parse, ParseError, Parser};
use moxy_token::span::Spanner;
use moxy_token::{Span, ToTokens, TokenStream};

use super::Type;
use crate::Delimited;

/// A slice type (e.g. `[T]`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TypeSlice {
    pub elem: Delimited<Box<Type>>,
}

impl Parse for TypeSlice {
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let elem = Delimited::parse_bracket_with(parser, |parser| Ok(Box::new(parser.parse::<Type>()?)))?;
        Ok(Self { elem })
    }
}

impl Spanner for TypeSlice {
    fn span(&self) -> Span {
        self.elem.span()
    }
}

impl ToTokens for TypeSlice {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.elem.to_tokens(tokens);
    }
}
