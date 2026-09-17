use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A struct/enum field definition (`pub name: Type` or `pub Type`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Field {
    pub attrs: Attributes,
    pub vis: Visibility,
    pub mutability: Mutability,
    pub ident: Option<Ident>,
    pub colon: Option<Token![:]>,
    pub ty: Type,
}

impl Parse for Field {
    fn peek(cursor: Cursor<'_>) -> bool {
        let cursor = Attributes::skip(cursor).unwrap_or(cursor);
        let cursor = Visibility::skip(cursor).unwrap_or(cursor);
        let cursor = Mutability::skip(cursor).unwrap_or(cursor);
        cursor.peek::<Type>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = parser.parse()?;
        let vis = parser.parse()?;
        let mutability = parser.parse()?;
        let (ident, colon) = if parser.peek::<Ident>() && parser.cursor().offset(1).peek::<Token![:]>() {
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
        cursor = Mutability::skip(cursor)?;

        if cursor.peek::<Ident>() && cursor.offset(1).peek::<Token![:]>() {
            cursor = cursor.skip::<Ident>()?;
            cursor = cursor.skip::<Token![:]>()?;
        }

        cursor.skip::<Type>()
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
