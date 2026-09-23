use crate::{Cursor, Parse, ParseError, Parser};
use moxy_token::span::Spanner;
use moxy_token::{Span, ToTokens, TokenStream};

use super::Type;
use crate::Delimited;

/// A slice type (e.g. `[T]`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TypeSlice {
    pub elem: Delimited<Box<Type>>,
}

impl Parse for TypeSlice {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(inner) = cursor.descend(moxy_token::Delim::Bracket) else {
            return false;
        };

        let Some(inner) = inner.skip::<Type>() else {
            return false;
        };

        inner.is_empty()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let elem = Delimited::parse_bracket_with(parser, |parser| Ok(Box::new(parser.parse()?)))?;
        Ok(Self { elem })
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
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
