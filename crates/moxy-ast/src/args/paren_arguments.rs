use moxy_token::{Delim, Span, Spanner, ToTokens, TokenStream};

use crate::{Cursor, Delimited, Parse, ParseError, Parser, Punctuated, ReturnType, Token, Type};

/// Parenthesized path arguments (`Fn(A, B) -> C`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ParenArguments {
    pub colon2: Option<Token![::]>,
    pub params: Delimited<Punctuated<Type, Token![,]>>,
    pub output: ReturnType,
}

impl Parse for ParenArguments {
    fn peek(cursor: Cursor<'_>) -> bool {
        let cursor = <Token![::]>::skip(cursor).unwrap_or(cursor);
        cursor.is_delimited(Delim::Paren)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let colon2 = parser.parse()?;
        let params = Delimited::parse_paren_with(parser, Punctuated::parse_terminated)?;
        let output = parser.parse()?;
        Ok(Self { colon2, params, output })
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        let mut cursor = <Token![::]>::skip(cursor).unwrap_or(cursor);
        let mut inner = cursor.descend(Delim::Paren)?;

        while !inner.is_empty() {
            inner = Type::skip(inner)?;

            if inner.is_empty() {
                break;
            }

            inner = <Token![,]>::skip(inner)?;
        }

        cursor = cursor.offset(1);
        ReturnType::skip(cursor)
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
