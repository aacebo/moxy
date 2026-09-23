use crate::{Cursor, Parse, ParseError, Parser};
use moxy_token::span::Spanner;
use moxy_token::{Span, ToTokens, TokenStream};

use super::Type;
use crate::Delimited;

/// A parenthesized type (e.g. `(T)`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TypeParen {
    pub content: Delimited<Box<Type>>,
}

impl Parse for TypeParen {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(inner) = cursor.descend(moxy_token::Delim::Paren) else {
            return false;
        };

        let Some(inner) = inner.skip::<Type>() else {
            return false;
        };

        inner.is_empty()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let content = Delimited::parse_paren_with(parser, |parser| Ok(Box::new(parser.parse()?)))?;
        Ok(Self { content })
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
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
