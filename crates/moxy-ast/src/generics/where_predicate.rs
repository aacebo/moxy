use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::{Parse, Span, Spanner, ToTokens, TokenStream};

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
            WherePredicate::Lifetime(v) => v.span(),
            WherePredicate::Type(v) => v.span(),
        }
    }
}

impl Parse for WherePredicate {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        if matches!(
            stream.curr(),
            Some(moxy_token::TokenTree::Punct(moxy_token::Punctuation::Quote(_)))
        ) {
            return Ok(WherePredicate::Lifetime(stream.parse()?));
        }

        Ok(WherePredicate::Type(Box::new(stream.parse()?)))
    }
}

impl ToTokens for WherePredicate {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            WherePredicate::Lifetime(v) => v.to_tokens(t),
            WherePredicate::Type(v) => v.to_tokens(t),
        }
    }
}
