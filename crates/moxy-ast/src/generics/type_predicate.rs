use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::{Colon, Plus};
use moxy_token::{Parse, Span, ToTokens, TokenStream};

use super::TypeBound;
use crate::{BoundLifetimes, Punctuated, Type};

#[doc = "A type predicate in a `where` clause (`T: Bound`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TypePredicate {
    pub span: Span,
    pub lifetimes: Option<BoundLifetimes>,
    pub bounded_ty: Type,
    pub colon_punct: Colon,
    pub bounds: Punctuated<TypeBound, Plus>,
}

impl Parse for TypePredicate {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let lifetimes = stream.parse_if::<BoundLifetimes>();
        let bounded_ty = stream.parse::<Type>()?;
        let colon_punct = stream.parse::<Colon>()?;
        let bounds = TypeBound::parse_bounds(stream)?;
        Ok(Self {
            span: Span::default(),
            lifetimes,
            bounded_ty,
            colon_punct,
            bounds,
        })
    }
}

impl ToTokens for TypePredicate {
    fn to_tokens(&self, t: &mut TokenStream) {
        if let Some(l) = &self.lifetimes {
            l.to_tokens(t);
        }

        self.bounded_ty.to_tokens(t);
        self.colon_punct.to_tokens(t);
        self.bounds.to_tokens(t);
    }
}
