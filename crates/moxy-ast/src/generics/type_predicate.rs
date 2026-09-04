use crate::Token;
use crate::{Parse, ParseError, Parser};
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use super::TypeBound;
use crate::{BoundLifetimes, Punctuated, Type};

/// A type predicate in a `where` clause (`T: Bound`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TypePredicate {
    pub lifetimes: Option<BoundLifetimes>,
    pub bounded_ty: Type,
    pub colon_punct: Token![:],
    pub bounds: Punctuated<TypeBound, Token![+]>,
}

impl Parse for TypePredicate {
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let lifetimes = parser.parse_if::<BoundLifetimes>();
        let bounded_ty = parser.parse::<Type>()?;
        let colon_punct = parser.parse::<Token![:]>()?;
        let bounds = TypeBound::parse_bounds(parser)?;

        Ok(Self {
            lifetimes,
            bounded_ty,
            colon_punct,
            bounds,
        })
    }
}

impl Spanner for TypePredicate {
    fn span(&self) -> Span {
        let start = if let Some(l) = &self.lifetimes {
            l.span()
        } else {
            self.bounded_ty.span()
        };

        let end = self
            .bounds
            .last()
            .map(|b| b.span())
            .unwrap_or_else(|| self.colon_punct.span());
        start.join(end)
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
