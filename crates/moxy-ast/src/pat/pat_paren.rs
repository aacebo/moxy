use moxy_token::{Delim, Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A parenthesized pattern, e.g. `(A | B)`.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatParen {
    pub attrs: Attributes,
    pub content: Delimited<Box<Pattern>>,
}

impl Spanner for PatParen {
    fn span(&self) -> Span {
        self.content.span()
    }
}

impl Parse for PatParen {
    fn peek(cursor: Cursor<'_>) -> bool {
        let cursor = Attributes::skip(cursor).unwrap_or(cursor);
        let Some(inner) = cursor.descend(Delim::Paren) else {
            return false;
        };

        let Some(inner) = Pattern::skip(inner) else {
            return false;
        };

        inner.is_empty()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = parser.parse()?;
        let content = Delimited::parse_paren_with(parser, |inner| Ok(Box::new(inner.parse()?)))?;
        Ok(Self { attrs, content })
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        let cursor = Attributes::skip(cursor)?;
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl ToTokens for PatParen {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.content.to_tokens(t);
    }
}
