use crate::Token;
use crate::{Parse, ParseError, Parser};
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use super::{ConstParam, LifetimeParam, TypeParam};

/// A generic parameter (lifetime, type, or const).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum GenericParam {
    Lifetime(LifetimeParam),
    Type(Box<TypeParam>),
    Const(Box<ConstParam>),
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

    pub fn as_lifetime(&self) -> Option<&LifetimeParam> {
        if let Self::Lifetime(v) = self { Some(v) } else { None }
    }

    pub fn as_type(&self) -> Option<&TypeParam> {
        if let Self::Type(v) = self { Some(v.as_ref()) } else { None }
    }

    pub fn as_const(&self) -> Option<&ConstParam> {
        if let Self::Const(v) = self { Some(v.as_ref()) } else { None }
    }
}

impl From<LifetimeParam> for GenericParam {
    fn from(v: LifetimeParam) -> Self {
        Self::Lifetime(v)
    }
}

impl From<TypeParam> for GenericParam {
    fn from(v: TypeParam) -> Self {
        Self::Type(Box::new(v))
    }
}

impl From<ConstParam> for GenericParam {
    fn from(v: ConstParam) -> Self {
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
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        if matches!(parser.curr(), Some(moxy_token::TokenTree::Punct(moxy_token::Punct::Quote(_)))) {
            return Ok(Self::Lifetime(parser.parse()?));
        }

        let fork = parser.lookahead();
        fork.skip_while::<crate::Attribute>();

        if fork.peek::<Token![const]>() {
            return Ok(Self::Const(Box::new(parser.parse()?)));
        }

        Ok(Self::Type(Box::new(parser.parse()?)))
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
