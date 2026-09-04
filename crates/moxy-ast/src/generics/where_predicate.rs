use crate::{Parse, ParseError, Parser};
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use super::{LifetimePredicate, TypePredicate};

/// A `where` clause predicate (lifetime or type).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum WherePredicate {
    Lifetime(LifetimePredicate),
    Type(Box<TypePredicate>),
}

impl Spanner for WherePredicate {
    fn span(&self) -> Span {
        match self {
            Self::Lifetime(v) => v.span(),
            Self::Type(v) => v.span(),
        }
    }
}

impl Parse for WherePredicate {
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        if matches!(parser.curr(), Some(moxy_token::TokenTree::Punct(moxy_token::Punct::Quote(_)))) {
            return Ok(Self::Lifetime(parser.parse()?));
        }

        Ok(Self::Type(Box::new(parser.parse()?)))
    }
}

impl ToTokens for WherePredicate {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            Self::Lifetime(v) => v.to_tokens(t),
            Self::Type(v) => v.to_tokens(t),
        }
    }
}
