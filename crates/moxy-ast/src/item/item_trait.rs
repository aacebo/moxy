use moxy_token::keyword::{Auto, Trait};
use moxy_token::parse::{ParseError, ParseStream};
use moxy_token::punct::{Colon, Plus};
use moxy_token::{Parse, Span, ToTokens, TokenStream};

use crate::{Attribute, Delimited, Generics, Ident, Punctuated, TraitItem, TypeBound, Unsafety, Visibility};

#[doc = "A trait definition item (`trait Name: Super { ... }`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemTrait {
    pub span: Span,
    pub attrs: Vec<Attribute>,
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
        let attrs = stream.parse::<Vec<Attribute>>()?;
        let vis = stream.parse::<Visibility>()?;
        let unsafety = stream.parse::<Unsafety>()?;
        let auto_keyword = if stream.peek::<Auto>().is_some() {
            Some(stream.parse::<Auto>()?)
        } else {
            None
        };

        let trait_keyword = stream.parse::<Trait>()?;
        let ident = stream.parse::<Ident>()?;
        let mut generics = stream.parse::<Generics>()?;

        let (colon_punct, supertraits) = if stream.peek::<Colon>().is_some() {
            let colon_punct = stream.parse::<Colon>()?;
            let supertraits = crate::TypeBound::parse_bounds(stream)?;
            (Some(colon_punct), supertraits)
        } else {
            (None, Punctuated::new())
        };

        if stream.peek::<moxy_token::keyword::Where>().is_some() {
            generics.where_clause = Some(stream.parse()?);
        }

        let items = Delimited::<Vec<TraitItem>>::parse_brace(stream)?;
        Ok(ItemTrait {
            span: Span::default(),
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

impl ToTokens for ItemTrait {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
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
