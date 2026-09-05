use crate::Token;
use crate::{Parse, ParseError, Parser};
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::{Attributes, Fields, Generics, Ident, Visibility};

/// A struct item (`struct Name<T> { ... }` or `struct Name(T);`).
#[derive(Debug, Clone, PartialEq, Eq)]
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
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = parser.parse::<Attributes>()?;
        let vis = parser.parse::<Visibility>()?;
        let struct_keyword = parser.parse::<Token![struct]>()?;
        let ident = parser.parse::<Ident>()?;
        let generics = parser.parse::<Generics>()?;
        let fields = parser.parse::<Fields>()?;
        let semi = parser.parse_if::<Token![;]>();

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
