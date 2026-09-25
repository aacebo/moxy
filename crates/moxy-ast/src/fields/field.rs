use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A struct/enum field definition (`pub name: Type` or `pub Type`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Field {
    pub attrs: Attributes,
    pub vis: Visibility,
    pub mutability: Option<Token![mut]>,
    pub ident: Option<Ident>,
    pub colon: Option<Token![:]>,
    pub ty: Type,
}

impl Parse for Field {
    fn peek(cursor: Cursor<'_>) -> bool {
        let cursor = Attributes::skip(cursor).unwrap_or(cursor);
        let cursor = Visibility::skip(cursor).unwrap_or(cursor);
        let cursor = Option::<Token![mut]>::skip(cursor).unwrap_or(cursor);
        Type::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = parser.parse()?;
        let vis = parser.parse()?;
        let mutability = parser.parse()?;
        let (ident, colon) = if Ident::peek(parser.cursor()) && <Token![:]>::peek(parser.cursor().offset(1)) {
            (Some(parser.parse()?), Some(parser.parse()?))
        } else {
            (None, None)
        };

        let ty = parser.parse()?;

        Ok(Self {
            attrs,
            vis,
            mutability,
            ident,
            colon,
            ty,
        })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = Attributes::skip(cursor)?;
        cursor = Visibility::skip(cursor)?;
        cursor = Option::<Token![mut]>::skip(cursor)?;

        if Ident::peek(cursor) && <Token![:]>::peek(cursor.offset(1)) {
            cursor = Ident::skip(cursor)?;
            cursor = <Token![:]>::skip(cursor)?;
        }

        Type::skip(cursor)
    }
}

impl Spanner for Field {
    fn span(&self) -> Span {
        self.attrs.span().join(self.ty.span())
    }
}

impl ToTokens for Field {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.vis.to_tokens(t);
        self.mutability.to_tokens(t);
        self.ident.to_tokens(t);
        self.colon.to_tokens(t);
        self.ty.to_tokens(t);
    }
}
