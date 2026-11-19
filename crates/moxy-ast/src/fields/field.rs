use moxy_token::{Ident, Span, Spanner, ToTokenStream, ToTokens, TokenStream};

use crate::*;

/// A struct/enum field definition (`pub name: Type` or `pub Type`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Field {
    pub attrs: Attributes,
    pub vis: Visibility,
    pub ident: Option<Ident>,
    pub colon: Option<Token![:]>,
    pub ty: Type,
}

impl Attributed for Field {
    fn attrs(&self) -> &[Attribute] {
        &self.attrs
    }
}

impl Parse for Field {
    fn peek(cursor: Cursor<'_>) -> bool {
        let cursor = Attributes::skip(cursor).unwrap_or(cursor);
        let cursor = Visibility::skip(cursor).unwrap_or(cursor);
        Type::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = <_ as Parse>::parse(parser)?;
        let vis = <_ as Parse>::parse(parser)?;
        let (ident, colon) = if Ident::peek(parser.cursor()) && peek_lone_colon(parser.cursor().offset(1)) {
            (Some(<_ as Parse>::parse(parser)?), Some(<_ as Parse>::parse(parser)?))
        } else {
            (None, None)
        };

        let ty = <_ as Parse>::parse(parser)?;

        Ok(Self {
            attrs,
            vis,
            ident,
            colon,
            ty,
        })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = Attributes::skip(cursor)?;
        cursor = Visibility::skip(cursor)?;
        if Ident::peek(cursor) && peek_lone_colon(cursor.offset(1)) {
            cursor = Ident::skip(cursor)?;
            cursor = <Token![:]>::skip(cursor)?;
        }

        Type::skip(cursor)
    }
}

impl Spanner for Field {
    fn span(&self) -> Span {
        self.to_token_stream().span()
    }
}

impl ToTokens for Field {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.vis.to_tokens(t);
        self.ident.to_tokens(t);
        self.colon.to_tokens(t);
        self.ty.to_tokens(t);
    }
}
