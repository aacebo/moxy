use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use super::{TraitBound, UseBound};
use crate::*;

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

    pub fn parse_bounds(parser: &Parser) -> Result<crate::Punctuated<Self, Token![+]>, ParseError> {
        let mut bounds = crate::Punctuated::new();

        loop {
            bounds.push_value(parser.parse()?);

            if parser.peek::<Token![+]>() {
                bounds.push_punct(parser.parse()?);
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
            Self::Trait(v) => v.span(),
            Self::Lifetime(v) => v.span(),
            Self::Use(v) => v.span(),
        }
    }
}

impl From<TraitBound> for TypeBound {
    fn from(v: TraitBound) -> Self {
        Self::Trait(v)
    }
}

impl From<UseBound> for TypeBound {
    fn from(v: UseBound) -> Self {
        Self::Use(v)
    }
}

impl Parse for TypeBound {
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.peek::<Lifetime>() || cursor.peek::<UseBound>() || cursor.peek::<TraitBound>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        if parser.peek::<Lifetime>() {
            return Ok(Self::Lifetime(parser.parse()?));
        }

        if parser.peek::<UseBound>() {
            return Ok(Self::Use(parser.parse()?));
        }

        Ok(Self::Trait(parser.parse()?))
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        if cursor.peek::<Lifetime>() {
            cursor.skip::<Lifetime>()
        } else if cursor.peek::<UseBound>() {
            cursor.skip::<UseBound>()
        } else {
            cursor.skip::<TraitBound>()
        }
    }
}

impl ToTokens for TypeBound {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            Self::Trait(v) => v.to_tokens(t),
            Self::Lifetime(v) => v.to_tokens(t),
            Self::Use(v) => v.to_tokens(t),
        }
    }
}
