use crate::{Parse, ParseError, Parser};
use moxy_token::Token;
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::{Delimited, Punctuated, ReturnType, ty};

/// Parenthesized path arguments (`Fn(A, B) -> C`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ParenArguments {
    pub params: Delimited<Punctuated<ty::Type, Token![,]>>,
    pub output: ReturnType,
}

impl Parse for ParenArguments {
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let params = Delimited::parse_paren_with(parser, Punctuated::parse_terminated)?;
        let output = parser.parse::<ReturnType>()?;
        Ok(Self { params, output })
    }
}

impl Spanner for ParenArguments {
    fn span(&self) -> Span {
        self.params.span().join(self.output.span())
    }
}

impl ToTokens for ParenArguments {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.params.to_tokens(tokens);
        self.output.to_tokens(tokens);
    }
}
