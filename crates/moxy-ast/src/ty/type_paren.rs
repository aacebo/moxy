use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::span::Spanner;
use moxy_token::{Parse, Span, ToTokens, TokenStream};

use super::Type;
use crate::Delimited;

/// A parenthesized type (e.g. `(T)`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TypeParen {
    pub content: Delimited<Box<Type>>,
}

impl Parse for TypeParen {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let content = Delimited::parse_paren_with(stream, |stream| Ok(Box::new(stream.parse::<Type>()?)))?;
        Ok(Self { content })
    }
}

impl Spanner for TypeParen {
    fn span(&self) -> Span {
        self.content.span()
    }
}

impl ToTokens for TypeParen {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.content.to_tokens(tokens);
    }
}
