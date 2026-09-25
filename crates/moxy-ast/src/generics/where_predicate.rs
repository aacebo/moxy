use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use super::{LifetimePredicate, TypePredicate};
use crate::*;

/// A `where` clause predicate (lifetime or type).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
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
    fn peek(cursor: Cursor<'_>) -> bool {
        LifetimePredicate::peek(cursor) || TypePredicate::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        if LifetimePredicate::peek(parser.cursor()) {
            return Ok(Self::Lifetime(<_ as Parse>::parse(parser)?));
        }

        Ok(Self::Type(Box::new(<_ as Parse>::parse(parser)?)))
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        if LifetimePredicate::peek(cursor) {
            LifetimePredicate::skip(cursor)
        } else {
            TypePredicate::skip(cursor)
        }
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
