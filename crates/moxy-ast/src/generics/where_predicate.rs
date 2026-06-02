use moxy_token::parse::{ParseError, ParseStream};
use moxy_token::token::ToTokens;
use moxy_token::{Parse, TokenStream};

use super::{LifetimePredicate, TypePredicate};

#[doc = "A `where` clause predicate (lifetime or type)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum WherePredicate {
    Lifetime(LifetimePredicate),
    Type(Box<TypePredicate>),
}

impl Parse for WherePredicate {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        if matches!(
            stream.curr(),
            Some(moxy_token::TokenTree::Token(moxy_token::Token::Punct(
                moxy_token::token::Punctuation::Quote(_)
            )))
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
