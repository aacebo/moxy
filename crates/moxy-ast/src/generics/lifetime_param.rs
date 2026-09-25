use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A lifetime parameter (`'a: 'b + 'c`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct LifetimeParam {
    pub attrs: Attributes,
    pub lifetime: Lifetime,
    pub colon_punct: Option<Token![:]>,
    pub bounds: Punctuated<Lifetime, Token![+]>,
}

impl Parse for LifetimeParam {
    fn peek(cursor: Cursor<'_>) -> bool {
        Attributes::skip(cursor).map(|cursor| Lifetime::peek(cursor)).unwrap_or(false)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = <_ as Parse>::parse(parser)?;
        let lifetime = <_ as Parse>::parse(parser)?;
        let (colon_punct, bounds) = if <Token![:]>::peek(parser.cursor()) {
            let colon_punct = Some(<_ as Parse>::parse(parser)?);
            let bounds = Punctuated::parse_separated_nonempty(parser)?;
            (colon_punct, bounds)
        } else {
            (None, Punctuated::new())
        };

        Ok(Self {
            attrs,
            lifetime,
            colon_punct,
            bounds,
        })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = Attributes::skip(cursor)?;
        cursor = Lifetime::skip(cursor)?;

        if <Token![:]>::peek(cursor) {
            cursor = <Token![:]>::skip(cursor)?;
            cursor = Lifetime::skip(cursor)?;

            while <Token![+]>::peek(cursor) {
                cursor = <Token![+]>::skip(cursor)?;
                cursor = Lifetime::skip(cursor)?;
            }
        }

        Some(cursor)
    }
}

impl Spanner for LifetimeParam {
    fn span(&self) -> Span {
        let end = self.bounds.last().map(|b| b.span()).unwrap_or_else(|| self.lifetime.span());
        self.attrs.span().join(end)
    }
}

impl ToTokens for LifetimeParam {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.lifetime.to_tokens(t);
        self.colon_punct.to_tokens(t);
        self.bounds.to_tokens(t);
    }
}
