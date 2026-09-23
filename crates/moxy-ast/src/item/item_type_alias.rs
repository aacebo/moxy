use crate::Token;
use crate::{Parse, ParseError, Parser};
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::{Attributes, Generics, Ident, Type, Visibility};

/// A type alias item (`type Name<T> = Type;`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemTypeAlias {
    pub attrs: Attributes,
    pub vis: Visibility,
    pub type_keyword: Token![type],
    pub ident: Ident,
    pub generics: Generics,
    pub eq_punct: Token![=],
    pub ty: Type,
    pub semi_punct: Token![;],
}

impl Parse for ItemTypeAlias {
    fn peek(cursor: crate::Cursor<'_>) -> bool {
        let cursor = Attributes::skip(cursor).unwrap_or(cursor);
        let cursor = Visibility::skip(cursor).unwrap_or(cursor);
        cursor.peek::<Token![type]>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = parser.parse()?;
        let vis = parser.parse()?;
        let type_keyword = parser.parse()?;
        let ident = parser.parse()?;
        let generics = parser.parse()?;
        let eq_punct = parser.parse()?;
        let ty = parser.parse()?;
        let semi_punct = parser.parse()?;

        Ok(Self {
            attrs,
            vis,
            type_keyword,
            ident,
            generics,
            eq_punct,
            ty,
            semi_punct,
        })
    }

    fn skip(mut cursor: crate::Cursor<'_>) -> Option<crate::Cursor<'_>> {
        cursor = Attributes::skip(cursor)?;
        cursor = Visibility::skip(cursor)?;
        cursor = cursor.skip::<Token![type]>()?;
        cursor = cursor.skip::<Ident>()?;
        cursor = Generics::skip(cursor)?;
        cursor = cursor.skip::<Token![=]>()?;
        cursor = cursor.skip::<Type>()?;
        cursor.skip::<Token![;]>()
    }
}

impl Spanner for ItemTypeAlias {
    fn span(&self) -> Span {
        self.attrs.span().join(self.semi_punct.span())
    }
}

impl ToTokens for ItemTypeAlias {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.vis.to_tokens(t);
        self.type_keyword.to_tokens(t);
        self.ident.to_tokens(t);
        self.generics.to_tokens(t);
        self.eq_punct.to_tokens(t);
        self.ty.to_tokens(t);
        self.semi_punct.to_tokens(t);
    }
}

impl ItemTypeAlias {
    pub fn into_item(self) -> super::Item {
        super::Item::from(self)
    }
}
