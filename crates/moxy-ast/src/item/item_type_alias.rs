use crate::Token;
use crate::{Parse, ParseError, Parser};
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::{Attributes, Generics, Ident, Type, Visibility};

/// A type alias item (`type Name<T> = Type;`).
#[derive(Debug, Clone, PartialEq, Eq)]
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
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = parser.parse::<Attributes>()?;
        let vis = parser.parse::<Visibility>()?;
        let type_keyword = parser.parse::<Token![type]>()?;
        let ident = parser.parse::<Ident>()?;
        let generics = parser.parse::<Generics>()?;
        let eq_punct = parser.parse::<Token![=]>()?;
        let ty = parser.parse::<Type>()?;
        let semi_punct = parser.parse::<Token![;]>()?;

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
