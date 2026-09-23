use moxy_token::{Span, Spanner, ToTokens, TokenStream};

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

impl Spanner for PatBox {
    fn span(&self) -> Span {
        self.attrs.span().join(self.pattern.span())
    }
}

impl Parse for PatBox {
    fn peek(cursor: Cursor<'_>) -> bool {
        let cursor = Attributes::skip(cursor).unwrap_or(cursor);
        cursor.peek::<Token![const]>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            attrs: parser.parse()?,
            keyword: parser.parse()?,
            pattern: parser.parse()?,
        })
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Attributes::skip(cursor)?.skip::<Token![const]>()?.skip::<StmtBlock>()
    }
}

impl ToTokens for PatBox {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.keyword.to_tokens(t);
        self.pattern.to_tokens(t);
    }
}
