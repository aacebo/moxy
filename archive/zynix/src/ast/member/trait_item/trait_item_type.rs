use super::TraitItem;
use crate::ast::{Attribute, Generics, Ident, Punctuated, Type, TypeBound};
use crate::parse::{ParseError, ParseStream};
use crate::token::keyword::Type as KwType;
use crate::token::punct::{Colon, Eq, Plus, Semi};
use crate::token::{LexError, ToTokens};
use crate::{Parse, Span, TokenStream};

#[doc = "An associated type inside a trait definition (`type Name: Bound = Default;`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TraitItemType {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub ident: Ident,
    pub generics: Generics,
    pub bounds: Punctuated<TypeBound, Plus>,
    pub default: Option<Type>,
}

impl Parse for TraitItemType {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let at = stream.span();
        let attrs = stream.parse_vec::<Attribute>()?;

        if stream.curr().and_then(|t| t.name()).as_deref() != Some("type") {
            return Err(LexError::new(at).message("expected trait type").into());
        }

        let _ = stream.parse::<KwType>()?;
        let ident = stream.parse::<Ident>()?;
        let generics = stream.parse::<Generics>()?;

        let bounds = if stream.peek::<Colon>().is_some() {
            let _ = stream.parse::<Colon>()?;
            crate::ast::TypeBound::parse_bounds(stream)?
        } else {
            Punctuated::new()
        };

        let default = if stream.peek::<Eq>().is_some() {
            let _ = stream.parse::<Eq>()?;
            Some(stream.parse::<Type>()?)
        } else {
            None
        };

        let _ = stream.parse::<Semi>();
        Ok(TraitItemType {
            span: Span::default(),
            attrs,
            ident,
            generics,
            bounds,
            default,
        })
    }
}

impl ToTokens for TraitItemType {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        KwType::default().to_tokens(t);
        self.ident.to_tokens(t);
        self.generics.to_tokens(t);

        if !self.bounds.is_empty() {
            Colon::default().to_tokens(t);
            self.bounds.to_tokens(t);
        }

        if let Some(d) = &self.default {
            Eq::default().to_tokens(t);
            d.to_tokens(t);
        }

        Semi::default().to_tokens(t);
    }
}
