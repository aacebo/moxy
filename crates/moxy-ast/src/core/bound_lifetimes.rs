use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A `for<'a, 'b>` higher-ranked lifetime binder.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct BoundLifetimes {
    pub for_keyword: Token![for],
    pub lt: Token![<],
    pub params: Punctuated<Lifetime, Token![,]>,
    pub gt: Token![>],
}

impl Parse for BoundLifetimes {
    fn peek(cursor: Cursor<'_>) -> bool {
        <Token![for]>::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let for_keyword = parser.parse()?;
        let lt = parser.parse()?;
        let params = Punctuated::parse_separated_nonempty(parser)?;
        let gt = parser.parse()?;

        Ok(Self {
            for_keyword,
            lt,
            params,
            gt,
        })
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        let mut cursor = <Token![for]>::skip(cursor)?;
        cursor = <Token![<]>::skip(cursor)?;
        cursor = Lifetime::skip(cursor)?;

        while <Token![,]>::peek(cursor) {
            cursor = <Token![,]>::skip(cursor)?;
            cursor = Lifetime::skip(cursor)?;
        }

        <Token![>]>::skip(cursor)
    }
}

impl Spanner for BoundLifetimes {
    fn span(&self) -> Span {
        self.for_keyword.span().join(self.gt.span())
    }
}

impl ToTokens for BoundLifetimes {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.for_keyword.to_tokens(t);
        self.lt.to_tokens(t);
        self.params.to_tokens(t);
        self.gt.to_tokens(t);
    }
}
