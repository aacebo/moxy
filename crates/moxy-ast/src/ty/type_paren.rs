use moxy_token::parse::{ParseError, ParseStream};
use moxy_token::span::Spanner;
use moxy_token::{Parse, Span, ToTokens, TokenStream};

use super::Type;
use crate::Delimited;

#[doc = "A parenthesized type (e.g. `(T)`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TypeParen {
    pub span: Span,
    pub paren: Delimited<Box<Type>>,
}

impl Parse for TypeParen {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let paren = Delimited::parse_paren_with(stream, |stream| Ok(Box::new(stream.parse::<Type>()?)))?;
        Ok(Self {
            span: paren.span(),
            paren,
        })
    }
}

impl ToTokens for TypeParen {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.paren.to_tokens(tokens);
    }
}
