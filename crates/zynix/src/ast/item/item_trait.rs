use crate::ast::{Attribute, Generics, Ident, Punctuated, TraitItem, TypeBound, Unsafety, Visibility};
use crate::parse::{ParseError, ParseStream};
use crate::token::keyword::{Auto, Trait};
use crate::token::punct::{Colon, Plus};
use crate::token::{Delim, ToTokens};
use crate::{Parse, Span, TokenStream};

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
            crate::ast::TypeBound::parse_bounds(stream)?
        } else {
            Punctuated::new()
        };

        if stream.peek::<crate::token::keyword::Where>().is_some() {
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

        t.extend_one(crate::TokenTree::Group(crate::token::Group::new(
            crate::token::Delim::Brace,
            inner,
        )));
    }
}
