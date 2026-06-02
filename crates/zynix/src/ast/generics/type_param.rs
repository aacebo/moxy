use super::TypeBound;
use crate::ast::{Attribute, Ident, Punctuated, Type};
use crate::parse::{ParseError, ParseStream};
use crate::token::ToTokens;
use crate::token::punct::{Colon, Eq, Plus};
use crate::{Parse, Span, TokenStream};

#[doc = "A type parameter (`T: Bound = Default`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TypeParam {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub ident: Ident,
    pub bounds: Punctuated<TypeBound, Plus>,
    pub default: Option<Type>,
}

impl Parse for TypeParam {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse_vec::<Attribute>()?;
        let ident = stream.parse::<Ident>()?;

        let bounds = if stream.peek::<Colon>().is_some() {
            let _ = stream.parse::<Colon>()?;
            TypeBound::parse_bounds(stream)?
        } else {
            Punctuated::new()
        };

        let default = if stream.peek::<Eq>().is_some() {
            let _ = stream.parse::<Eq>()?;
            Some(stream.parse::<Type>()?)
        } else {
            None
        };

        Ok(Self {
            span: Span::default(),
            attrs,
            ident,
            bounds,
            default,
        })
    }
}

impl ToTokens for TypeParam {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.ident.to_tokens(t);
        if !self.bounds.is_empty() {
            Colon::default().to_tokens(t);
            self.bounds.to_tokens(t);
        }
        if let Some(d) = &self.default {
            Eq::default().to_tokens(t);
            d.to_tokens(t);
        }
    }
}
