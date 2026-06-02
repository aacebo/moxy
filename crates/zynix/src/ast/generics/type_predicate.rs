use super::TypeBound;
use crate::ast::{BoundLifetimes, Punctuated, Type};
use crate::parse::{ParseError, ParseStream};
use crate::token::ToTokens;
use crate::token::punct::{Colon, Plus};
use crate::{Parse, Span, TokenStream};

#[doc = "A type predicate in a `where` clause (`T: Bound`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TypePredicate {
    pub span: Span,
    pub lifetimes: Option<BoundLifetimes>,
    pub bounded_ty: Type,
    pub bounds: Punctuated<TypeBound, Plus>,
}

impl Parse for TypePredicate {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let lifetimes = stream.parse_opt::<BoundLifetimes>();
        let bounded_ty = stream.parse::<Type>()?;
        let _ = stream.parse::<Colon>()?;
        let bounds = TypeBound::parse_bounds(stream)?;
        Ok(Self {
            span: Span::default(),
            lifetimes,
            bounded_ty,
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
        Colon::default().to_tokens(t);
        self.bounds.to_tokens(t);
    }
}
