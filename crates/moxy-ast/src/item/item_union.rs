use crate::Token;
use crate::{Parse, ParseError, Parser};
use moxy_token::{Ident, Span, Spanner, ToTokens, TokenStream};

use crate::{Attributes, FieldsNamed, Generics, Visibility};

/// A union item (`union Name<T> { field: Type, ... }`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemUnion {
    pub attrs: Attributes,
    pub vis: Visibility,
    pub union_keyword: Token![union],
    pub ident: Ident,
    pub generics: Generics,
    pub fields: FieldsNamed,
}

impl Parse for ItemUnion {
    fn peek(cursor: crate::Cursor<'_>) -> bool {
        let cursor = Attributes::skip(cursor).unwrap_or(cursor);
        let cursor = Visibility::skip(cursor).unwrap_or(cursor);
        <Token![union]>::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = <_ as Parse>::parse(parser)?;
        let vis = <_ as Parse>::parse(parser)?;
        let union_keyword = <_ as Parse>::parse(parser)?;
        let ident = <_ as Parse>::parse(parser)?;
        let generics = <_ as Parse>::parse(parser)?;
        let fields = <_ as Parse>::parse(parser)?;

        Ok(Self {
            attrs,
            vis,
            union_keyword,
            ident,
            generics,
            fields,
        })
    }

    fn skip(mut cursor: crate::Cursor<'_>) -> Option<crate::Cursor<'_>> {
        cursor = Attributes::skip(cursor)?;
        cursor = Visibility::skip(cursor)?;
        cursor = <Token![union]>::skip(cursor)?;
        cursor = Ident::skip(cursor)?;
        cursor = Generics::skip(cursor)?;
        FieldsNamed::skip(cursor)
    }
}

impl Spanner for ItemUnion {
    fn span(&self) -> Span {
        self.attrs.span().join(self.fields.span())
    }
}

impl ToTokens for ItemUnion {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.vis.to_tokens(t);
        self.union_keyword.to_tokens(t);
        self.ident.to_tokens(t);
        self.generics.to_tokens(t);
        self.fields.to_tokens(t);
    }
}

impl ItemUnion {
    pub fn into_item(self) -> super::Item {
        super::Item::from(self)
    }
}
