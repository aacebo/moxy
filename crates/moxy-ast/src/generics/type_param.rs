use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::{Colon, Eq, Plus};
use moxy_token::{Parse, Span, Spanner, ToTokens, TokenStream};

use super::TypeBound;
use crate::{Attribute, Ident, Punctuated, Type};

#[doc = "A type parameter (`T: Bound = Default`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TypeParam {
    pub attrs: Vec<Attribute>,
    pub ident: Ident,
    pub colon_punct: Option<Colon>,
    pub bounds: Punctuated<TypeBound, Plus>,
    pub eq_punct: Option<Eq>,
    pub default: Option<Type>,
}

impl Parse for TypeParam {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse::<Vec<Attribute>>()?;
        let ident = stream.parse::<Ident>()?;

        let (colon_punct, bounds) = if stream.peek::<Colon>().is_some() {
            let colon_punct = stream.parse::<Colon>()?;
            let bounds = TypeBound::parse_bounds(stream)?;
            (Some(colon_punct), bounds)
        } else {
            (None, Punctuated::new())
        };

        let (eq_punct, default) = if stream.peek::<Eq>().is_some() {
            let eq_punct = stream.parse::<Eq>()?;
            let default = stream.parse::<Type>()?;
            (Some(eq_punct), Some(default))
        } else {
            (None, None)
        };

        Ok(Self {
            attrs,
            ident,
            colon_punct,
            bounds,
            eq_punct,
            default,
        })
    }
}

impl Spanner for TypeParam {
    fn span(&self) -> Span {
        let start = if let Some(a) = self.attrs.first() {
            a.span()
        } else {
            self.ident.span
        };
        let end = if let Some(d) = &self.default {
            d.span()
        } else if let Some(b) = self.bounds.last() {
            b.span()
        } else {
            self.ident.span
        };
        start.join(end)
    }
}

impl ToTokens for TypeParam {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.ident.to_tokens(t);
        if !self.bounds.is_empty() {
            if let Some(colon_punct) = &self.colon_punct {
                colon_punct.to_tokens(t);
            }
            self.bounds.to_tokens(t);
        }
        if let Some(d) = &self.default {
            if let Some(eq_punct) = &self.eq_punct {
                eq_punct.to_tokens(t);
            }
            d.to_tokens(t);
        }
    }
}
