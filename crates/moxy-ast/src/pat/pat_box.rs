use moxy_token::{Span, Spanner, ToTokenStream, ToTokens, TokenStream};

use crate::*;

/// A const pattern, e.g. `box `.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatBox {
    pub attrs: Attributes,
    pub keyword: Token![box],
    pub pattern: Box<Pattern>,
}

impl Attributed for PatBox {
    fn attrs(&self) -> &[Attribute] {
        &self.attrs
    }
}

impl Spanner for PatBox {
    fn span(&self) -> Span {
        self.to_token_stream().span()
    }
}

impl Parse for PatBox {
    fn peek(cursor: Cursor<'_>) -> bool {
        let cursor = Attributes::skip(cursor).unwrap_or(cursor);
        <Token![box]>::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            attrs: <_ as Parse>::parse(parser)?,
            keyword: <_ as Parse>::parse(parser)?,
            pattern: Box::new(super::parse::single(parser)?),
        })
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        super::skip::single(<Token![box]>::skip(Attributes::skip(cursor)?)?)
    }
}

impl ToTokens for PatBox {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.keyword.to_tokens(t);
        self.pattern.to_tokens(t);
    }
}
