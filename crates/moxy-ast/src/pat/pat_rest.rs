use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A spread pattern, e.g. `..`.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatRest {
    pub attrs: Attributes,
    pub token: Token![..],
}

impl Spanner for PatRest {
    fn span(&self) -> Span {
        self.attrs.span().join(self.token.span())
    }
}

impl Parse for PatRest {
    fn peek(cursor: Cursor<'_>) -> bool {
        let cursor = Attributes::skip(cursor).unwrap_or(cursor);
        <Token![..]>::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            attrs: parser.parse()?,
            token: parser.parse()?,
        })
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        <Token![..]>::skip(Attributes::skip(cursor)?)
    }
}

impl ToTokens for PatRest {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.token.to_tokens(t);
    }
}
