use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use super::{TraitBound, UseBound};
use crate::*;

/// A bound on a type parameter (`Trait`, `'a`, `use<>`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
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
        bounds.push_value(<_ as Parse>::parse(parser)?);

        while <Token![+]>::peek(parser.cursor()) {
            bounds.push_punct(<_ as Parse>::parse(parser)?);

            if TypeBound::peek(parser.cursor()) {
                bounds.push_value(<_ as Parse>::parse(parser)?);
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
        Lifetime::peek(cursor) || UseBound::peek(cursor) || TraitBound::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        if Lifetime::peek(parser.cursor()) {
            return Ok(Self::Lifetime(<_ as Parse>::parse(parser)?));
        }

        if UseBound::peek(parser.cursor()) {
            return Ok(Self::Use(<_ as Parse>::parse(parser)?));
        }

        Ok(Self::Trait(<_ as Parse>::parse(parser)?))
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        if Lifetime::peek(cursor) {
            Lifetime::skip(cursor)
        } else if UseBound::peek(cursor) {
            UseBound::skip(cursor)
        } else {
            TraitBound::skip(cursor)
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
