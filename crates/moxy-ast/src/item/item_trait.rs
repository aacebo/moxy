use moxy_token::keyword::{Auto, Trait};
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::{Colon, Plus};
use moxy_token::{Parse, Span, Spanner, ToTokens, TokenStream};

use crate::{Attributes, Delimited, Generics, Ident, Punctuated, TraitItem, TypeBound, Unsafety, Visibility};

/// A trait definition item (`trait Name: Super { ... }`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemTrait {
    pub attrs: Attributes,
    pub vis: Visibility,
    pub unsafety: Unsafety,
    pub auto_keyword: Option<Auto>,
    pub trait_keyword: Trait,
    pub ident: Ident,
    pub generics: Generics,
    pub colon_punct: Option<Colon>,
    pub supertraits: Punctuated<TypeBound, Plus>,
    pub items: Delimited<Vec<TraitItem>>,
}

impl Parse for ItemTrait {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse::<Attributes>()?;
        let vis = stream.parse::<Visibility>()?;
        let unsafety = stream.parse::<Unsafety>()?;
        let auto_keyword = if stream.peek::<Auto>() {
            Some(stream.parse::<Auto>()?)
        } else {
            None
        };

        let trait_keyword = stream.parse::<Trait>()?;
        let ident = stream.parse::<Ident>()?;
        let mut generics = stream.parse::<Generics>()?;
        let (colon_punct, supertraits) = if stream.peek::<Colon>() {
            let colon_punct = stream.parse::<Colon>()?;
            let supertraits = crate::TypeBound::parse_bounds(stream)?;
            (Some(colon_punct), supertraits)
        } else {
            (None, Punctuated::new())
        };

        if stream.peek::<moxy_token::keyword::Where>() {
            generics.where_clause = Some(stream.parse()?);
        }

        let items = Delimited::<Vec<TraitItem>>::parse_brace(stream)?;

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

        if let Some(auto_keyword) = &self.auto_keyword {
            auto_keyword.to_tokens(t);
        }

        self.trait_keyword.to_tokens(t);
        self.ident.to_tokens(t);
        self.generics.to_tokens(t);

        if !self.supertraits.is_empty() {
            if let Some(colon_punct) = &self.colon_punct {
                colon_punct.to_tokens(t);
            }
            self.supertraits.to_tokens(t);
        }

        self.items.to_tokens(t);
    }
}

impl ItemTrait {
    pub fn into_item(self) -> super::Item {
        super::Item::from(self)
    }
}
