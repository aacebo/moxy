use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A wild pattern, e.g. `_`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatWild {
    pub attrs: Attributes,
    pub token: Token![_],
}

impl Spanner for PatWild {
    fn span(&self) -> Span {
        self.attrs.span().join(self.token.span())
    }
}

impl Parse for PatWild {
    fn peek(cursor: Cursor<'_>) -> bool {
        let cursor = Attributes::skip(cursor).unwrap_or(cursor);
        cursor.peek::<Token![_]>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            attrs: parser.parse()?,
            token: parser.parse()?,
        })
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Attributes::skip(cursor)?.skip::<Token![_]>()
    }
}

impl ToTokens for PatWild {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.token.to_tokens(t);
    }
}
