use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::span::Spanner;
use moxy_token::{Parse, Span, ToTokens, TokenStream};

use super::Type;
use crate::Delimited;

#[doc = "A slice type (e.g. `[T]`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TypeSlice {
    pub span: Span,
    pub elem: Delimited<Box<Type>>,
}

impl Parse for TypeSlice {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let elem = Delimited::parse_bracket_with(stream, |stream| Ok(Box::new(stream.parse::<Type>()?)))?;
        Ok(Self { span: elem.span(), elem })
    }
}

impl ToTokens for TypeSlice {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.elem.to_tokens(tokens);
    }
}
