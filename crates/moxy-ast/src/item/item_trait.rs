use crate::Token;
use crate::{Parse, ParseError, Parser};
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::{Attributes, Delimited, Generics, Ident, Punctuated, TraitItem, TypeBound, Unsafety, Visibility};

/// A trait definition item (`trait Name: Super { ... }`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemTrait {
    pub attrs: Attributes,
    pub vis: Visibility,
    pub unsafety: Unsafety,
    pub auto_keyword: Option<Token![auto]>,
    pub trait_keyword: Token![trait],
    pub ident: Ident,
    pub generics: Generics,
    pub colon_punct: Option<Token![:]>,
    pub supertraits: Punctuated<TypeBound, Token![+]>,
    pub items: Delimited<Vec<TraitItem>>,
}

impl Parse for ItemTrait {
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = parser.parse::<Attributes>()?;
        let vis = parser.parse::<Visibility>()?;
        let unsafety = parser.parse::<Unsafety>()?;
        let auto_keyword = if parser.peek::<Token![auto]>() {
            Some(parser.parse::<Token![auto]>()?)
        } else {
            None
        };

        let trait_keyword = parser.parse::<Token![trait]>()?;
        let ident = parser.parse::<Ident>()?;
        let mut generics = parser.parse::<Generics>()?;
        let (colon_punct, supertraits) = if parser.peek::<Token![:]>() {
            let colon_punct = parser.parse::<Token![:]>()?;
            let supertraits = crate::TypeBound::parse_bounds(parser)?;
            (Some(colon_punct), supertraits)
        } else {
            (None, Punctuated::new())
        };

        generics.where_clause = parser.parse_if();
        let items = Delimited::<Vec<TraitItem>>::parse_brace(parser)?;

        Ok(Self {
            attrs,
            vis,
            unsafety,
            auto_keyword,
            trait_keyword,
            ident,
            generics,
            colon_punct,
            supertraits,
            items,
        })
    }
}

impl Spanner for ItemTrait {
    fn span(&self) -> Span {
        self.attrs.span().join(self.items.span())
    }
}

impl ToTokens for ItemTrait {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.vis.to_tokens(t);
        self.unsafety.to_tokens(t);
        self.auto_keyword.to_tokens(t);
        self.trait_keyword.to_tokens(t);
        self.ident.to_tokens(t);
        self.generics.to_tokens(t);
        self.colon_punct.to_tokens(t);
        self.supertraits.to_tokens(t);
        self.items.to_tokens(t);
    }
}

impl ItemTrait {
    pub fn into_item(self) -> super::Item {
        super::Item::from(self)
    }
}
