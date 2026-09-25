use moxy_token::{Ident, Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A C-style variadic marker (`...`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Variadic {
    pub attrs: Attributes,
    pub name: Option<Ident>,
    pub dots: Token![...],
}

impl Parse for Variadic {
    fn peek(cursor: Cursor<'_>) -> bool {
        let cursor = Attributes::skip(cursor).unwrap_or(cursor);
        <Token![...]>::peek(cursor) || (Ident::peek(cursor) && <Token![...]>::peek(cursor.offset(1)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            attrs: <_ as Parse>::parse(parser)?,
            name: <_ as Parse>::parse(parser)?,
            dots: <_ as Parse>::parse(parser)?,
        })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = Attributes::skip(cursor)?;
        cursor = Option::<Ident>::skip(cursor)?;
        <Token![...]>::skip(cursor)
    }
}

impl Spanner for Variadic {
    fn span(&self) -> Span {
        self.attrs.span().join(self.dots.span())
    }
}

impl ToTokens for Variadic {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.name.to_tokens(t);
        self.dots.to_tokens(t);
    }
}
