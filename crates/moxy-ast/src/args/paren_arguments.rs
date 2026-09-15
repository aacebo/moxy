use moxy_token::{Delim, Group, Span, Spanner, ToTokens, TokenStream};

use crate::{Cursor, Delimited, Parse, ParseError, Parser, Punctuated, ReturnType, Token, Type};

/// Parenthesized path arguments (`Fn(A, B) -> C`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ParenArguments {
    pub params: Delimited<Punctuated<Type, Token![,]>>,
    pub output: ReturnType,
}

impl Parse for ParenArguments {
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.peek::<Group>() && cursor.descend(Delim::Paren).map(|c| c.peek::<Type>()).unwrap_or_default()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let params = Delimited::parse_paren_with(parser, Punctuated::parse_terminated)?;
        let output = parser.parse()?;
        Ok(Self { params, output })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = cursor.skip::<Delimited<Punctuated<Type, Token![,]>>>()?;
        cursor.skip::<ReturnType>()
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
