use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A generic parameter (lifetime, type, or const).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum GenericParam {
    Lifetime(generics::LifetimeParam),
    Type(Box<generics::TypeParam>),
    Const(Box<generics::ConstParam>),
}

impl GenericParam {
    pub fn is_lifetime(&self) -> bool {
        matches!(self, Self::Lifetime(_))
    }

    pub fn is_type(&self) -> bool {
        matches!(self, Self::Type(_))
    }

    pub fn is_const(&self) -> bool {
        matches!(self, Self::Const(_))
    }

    pub fn as_lifetime(&self) -> Option<&generics::LifetimeParam> {
        if let Self::Lifetime(v) = self { Some(v) } else { None }
    }

    pub fn as_type(&self) -> Option<&generics::TypeParam> {
        if let Self::Type(v) = self { Some(v.as_ref()) } else { None }
    }

    pub fn as_const(&self) -> Option<&generics::ConstParam> {
        if let Self::Const(v) = self { Some(v.as_ref()) } else { None }
    }
}

impl From<generics::LifetimeParam> for GenericParam {
    fn from(v: generics::LifetimeParam) -> Self {
        Self::Lifetime(v)
    }
}

impl From<generics::TypeParam> for GenericParam {
    fn from(v: generics::TypeParam) -> Self {
        Self::Type(Box::new(v))
    }
}

impl From<generics::ConstParam> for GenericParam {
    fn from(v: generics::ConstParam) -> Self {
        Self::Const(Box::new(v))
    }
}

impl Spanner for GenericParam {
    fn span(&self) -> Span {
        match self {
            Self::Lifetime(v) => v.span(),
            Self::Type(v) => v.span(),
            Self::Const(v) => v.span(),
        }
    }
}

impl Parse for GenericParam {
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.peek::<generics::LifetimeParam>() || cursor.peek::<generics::ConstParam>() || cursor.peek::<generics::TypeParam>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        if parser.peek::<generics::LifetimeParam>() {
            return Ok(Self::Lifetime(parser.parse()?));
        }

        if parser.peek::<generics::ConstParam>() {
            return Ok(Self::Const(Box::new(parser.parse()?)));
        }

        Ok(Self::Type(Box::new(parser.parse()?)))
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        if cursor.peek::<generics::LifetimeParam>() {
            cursor.skip::<generics::LifetimeParam>()
        } else if cursor.peek::<generics::ConstParam>() {
            cursor.skip::<generics::ConstParam>()
        } else {
            cursor.skip::<generics::TypeParam>()
        }
    }
}

impl ToTokens for GenericParam {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            Self::Lifetime(v) => v.to_tokens(t),
            Self::Type(v) => v.to_tokens(t),
            Self::Const(v) => v.to_tokens(t),
        }
    }
}
