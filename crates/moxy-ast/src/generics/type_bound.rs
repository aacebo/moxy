use moxy_token::parse::{ParseError, ParseStream};
use moxy_token::token::ToTokens;
use moxy_token::{Parse, TokenStream};

use super::{TraitBound, UseBound};
use crate::Lifetime;

#[doc = "A bound on a type parameter (`Trait`, `'a`, `use<>`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum TypeBound {
    Trait(TraitBound),
    Lifetime(Lifetime),
    Use(UseBound),
}

impl TypeBound {
    pub fn parse_bounds(
        stream: &mut moxy_token::parse::ParseStream,
    ) -> Result<crate::Punctuated<Self, moxy_token::token::punct::Plus>, moxy_token::parse::ParseError> {
        use moxy_token::token::punct::Plus;
        let mut bounds = crate::Punctuated::new();

        loop {
            bounds.push_value(stream.parse::<TypeBound>()?);

            if stream.peek::<Plus>().is_some() {
                bounds.push_punct(stream.parse::<Plus>()?);
            } else {
                break;
            }
        }

        Ok(bounds)
    }
}

impl From<TraitBound> for TypeBound {
    fn from(v: TraitBound) -> Self {
        TypeBound::Trait(v)
    }
}

impl From<UseBound> for TypeBound {
    fn from(v: UseBound) -> Self {
        TypeBound::Use(v)
    }
}

impl Parse for TypeBound {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        if matches!(
            stream.curr(),
            Some(moxy_token::TokenTree::Token(moxy_token::Token::Punct(
                moxy_token::token::Punctuation::Quote(_)
            )))
        ) {
            return Ok(TypeBound::Lifetime(stream.parse()?));
        }

        if stream.peek::<moxy_token::token::keyword::Use>().is_some() {
            return Ok(TypeBound::Use(stream.parse()?));
        }

        Ok(TypeBound::Trait(stream.parse()?))
    }
}

impl ToTokens for TypeBound {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            TypeBound::Trait(v) => v.to_tokens(t),
            TypeBound::Lifetime(v) => v.to_tokens(t),
            TypeBound::Use(v) => v.to_tokens(t),
        }
    }
}
