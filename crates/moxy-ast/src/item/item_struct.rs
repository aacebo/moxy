use crate::Token;
use crate::{Parse, ParseError, Parser};
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::{Attributes, Fields, Generics, Ident, Visibility};

/// A struct item (`struct Name<T> { ... }` or `struct Name(T);`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemStruct {
    pub attrs: Attributes,
    pub vis: Visibility,
    pub struct_keyword: Token![struct],
    pub ident: Ident,
    pub generics: Generics,
    pub fields: Fields,
    pub semi: Option<Token![;]>,
}

impl Parse for ItemStruct {
    fn peek(cursor: crate::Cursor<'_>) -> bool {
        let cursor = Attributes::skip(cursor).unwrap_or(cursor);
        let cursor = Visibility::skip(cursor).unwrap_or(cursor);
        <Token![struct]>::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = <_ as Parse>::parse(parser)?;
        let vis = <_ as Parse>::parse(parser)?;
        let struct_keyword = <_ as Parse>::parse(parser)?;
        let ident = <_ as Parse>::parse(parser)?;
        let generics = <_ as Parse>::parse(parser)?;
        let fields = <_ as Parse>::parse(parser)?;
        let semi = <_ as Parse>::parse(parser)?;

        Ok(Self {
            attrs,
            vis,
            struct_keyword,
            ident,
            generics,
            fields,
            semi,
        })
    }

    fn skip(mut cursor: crate::Cursor<'_>) -> Option<crate::Cursor<'_>> {
        cursor = Attributes::skip(cursor)?;
        cursor = Visibility::skip(cursor)?;
        cursor = <Token![struct]>::skip(cursor)?;
        cursor = Ident::skip(cursor)?;
        cursor = Generics::skip(cursor)?;
        cursor = Fields::skip(cursor)?;
        Option::<Token![;]>::skip(cursor)
    }
}

impl Spanner for ItemStruct {
    fn span(&self) -> Span {
        self.attrs.span().join(self.fields.span())
    }
}

impl ToTokens for ItemStruct {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.vis.to_tokens(t);
        self.struct_keyword.to_tokens(t);
        self.ident.to_tokens(t);
        self.generics.to_tokens(t);
        self.fields.to_tokens(t);
        self.semi.to_tokens(t);
    }
}

impl ItemStruct {
    pub fn into_item(self) -> super::Item {
        super::Item::from(self)
    }
}
