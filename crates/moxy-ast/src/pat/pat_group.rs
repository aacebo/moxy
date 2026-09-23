use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A grouped pattern (used internally by the parser for grouping token trees).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatGroup {
    pub attrs: Attributes,
    pub pat: Box<Pattern>,
}

impl Spanner for PatGroup {
    fn span(&self) -> Span {
        self.attrs.span().join(self.pat.span())
    }
}

impl Parse for PatGroup {
    fn peek(cursor: Cursor<'_>) -> bool {
        let cursor = Attributes::skip(cursor).unwrap_or(cursor);
        let Some(inner) = cursor.descend(moxy_token::Delim::None) else {
            return false;
        };

        let Some(inner) = inner.skip::<Pattern>() else {
            return false;
        };

        inner.is_empty()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = parser.parse()?;
        let inner = parser.parse_group(moxy_token::Delim::None)?;
        Ok(Self {
            attrs,
            pat: inner.parse()?,
        })
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        let cursor = Attributes::skip(cursor)?;
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl ToTokens for PatGroup {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.pat.to_tokens(t);
    }
}
