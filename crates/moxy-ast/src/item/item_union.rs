use crate::Token;
use crate::{Parse, ParseError, Parser};
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::{Attributes, FieldsNamed, Generics, Ident, Visibility};

/// A union item (`union Name<T> { field: Type, ... }`).
#[derive(Debug, Clone, PartialEq, Eq)]
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
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = parser.parse::<Attributes>()?;
        let vis = parser.parse::<Visibility>()?;
        let union_keyword = parser.parse::<Token![union]>()?;
        let ident = parser.parse::<Ident>()?;
        let generics = parser.parse::<Generics>()?;
        let fields = parser.parse::<FieldsNamed>()?;

        Ok(Self {
            attrs,
            vis,
            union_keyword,
            ident,
            generics,
            fields,
        })
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
