use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::{Parse, Span, Spanner, ToTokens, TokenStream};

use super::{TraitBound, UseBound};
use crate::Lifetime;

/// A bound on a type parameter (`Trait`, `'a`, `use<>`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum TypeBound {
    Trait(TraitBound),
    Lifetime(Lifetime),
    Use(UseBound),
}

impl TypeBound {
    pub fn is_trait(&self) -> bool {
        matches!(self, Self::Trait(_))
    }

    pub fn is_lifetime(&self) -> bool {
        matches!(self, Self::Lifetime(_))
    }

    pub fn is_use(&self) -> bool {
        matches!(self, Self::Use(_))
    }

    pub fn as_trait(&self) -> Option<&TraitBound> {
        if let Self::Trait(v) = self { Some(v) } else { None }
    }

    pub fn as_lifetime(&self) -> Option<&Lifetime> {
        if let Self::Lifetime(v) = self { Some(v) } else { None }
    }

    pub fn as_use(&self) -> Option<&UseBound> {
        if let Self::Use(v) = self { Some(v) } else { None }
    }

    pub fn parse_bounds(
        stream: &mut moxy_token::parser::ParseStream,
    ) -> Result<crate::Punctuated<Self, moxy_token::punct::Plus>, moxy_token::parser::ParseError> {
        use moxy_token::punct::Plus;
        let mut bounds = crate::Punctuated::new();

        loop {
            bounds.push_value(stream.parse::<TypeBound>()?);

            if stream.peek::<Plus>() {
                bounds.push_punct(stream.parse::<Plus>()?);
            } else {
                break;
            }
        }

        Ok(bounds)
    }
}

impl Spanner for TypeBound {
    fn span(&self) -> Span {
        match self {
            TypeBound::Trait(v) => v.span(),
            TypeBound::Lifetime(v) => v.span(),
            TypeBound::Use(v) => v.span(),
        }
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
            Some(moxy_token::TokenTree::Punct(moxy_token::Punctuation::Quote(_)))
        ) {
            return Ok(TypeBound::Lifetime(stream.parse()?));
        }

        if stream.peek::<moxy_token::keyword::Use>() {
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
