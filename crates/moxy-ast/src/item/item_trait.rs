use moxy_token::parse::{ParseError, ParseStream};
use moxy_token::token::keyword::{Auto, Trait};
use moxy_token::token::punct::{Colon, Plus};
use moxy_token::token::{Delim, ToTokens};
use moxy_token::{Parse, Span, TokenStream};

use crate::{Attribute, Generics, Ident, Punctuated, TraitItem, TypeBound, Unsafety, Visibility};

#[doc = "A trait definition item (`trait Name: Super { ... }`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemTrait {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub unsafety: Unsafety,
    pub auto: bool,
    pub ident: Ident,
    pub generics: Generics,
    pub supertraits: Punctuated<TypeBound, Plus>,
    pub items: Vec<TraitItem>,
}

impl Parse for ItemTrait {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse_vec::<Attribute>()?;
        let vis = stream.parse::<Visibility>()?;
        let unsafety = stream.parse::<Unsafety>()?;
        let auto = if stream.peek::<Auto>().is_some() {
            let _ = stream.parse::<Auto>()?;
            true
        } else {
            false
        };

        let _ = stream.parse::<Trait>()?;
        let ident = stream.parse::<Ident>()?;
        let mut generics = stream.parse::<Generics>()?;

        let supertraits = if stream.peek::<Colon>().is_some() {
            let _ = stream.parse::<Colon>()?;
            crate::TypeBound::parse_bounds(stream)?
        } else {
            Punctuated::new()
        };

        if stream.peek::<moxy_token::token::keyword::Where>().is_some() {
            generics.where_clause = Some(stream.parse()?);
        }

        let group = stream.parse_group(Delim::Brace)?;
        let mut inner = group.parse();
        let items = inner.parse_vec::<TraitItem>()?;
        Ok(ItemTrait {
            span: Span::default(),
            attrs,
            vis,
            unsafety,
            auto,
            ident,
            generics,
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

        if self.auto {
            Auto::default().to_tokens(t);
        }

        Trait::default().to_tokens(t);
        self.ident.to_tokens(t);
        self.generics.to_tokens(t);

        if !self.supertraits.is_empty() {
            Colon::default().to_tokens(t);
            self.supertraits.to_tokens(t);
        }

        let mut inner = TokenStream::new();

        for it in &self.items {
            it.to_tokens(&mut inner);
        }

        t.extend_one(moxy_token::TokenTree::Group(moxy_token::token::Group::new(
            moxy_token::token::Delim::Brace,
            inner,
        )));
    }
}
