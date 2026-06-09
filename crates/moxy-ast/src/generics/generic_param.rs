use moxy_token::keyword::Const;
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::{Parse, Span, Spanner, ToTokens, TokenStream};

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
        GenericParam::Lifetime(v)
    }
}

impl From<TypeParam> for GenericParam {
    fn from(v: TypeParam) -> Self {
        GenericParam::Type(Box::new(v))
    }
}

impl From<ConstParam> for GenericParam {
    fn from(v: ConstParam) -> Self {
        GenericParam::Const(Box::new(v))
    }
}

impl Spanner for GenericParam {
    fn span(&self) -> Span {
        match self {
            GenericParam::Lifetime(v) => v.span(),
            GenericParam::Type(v) => v.span(),
            GenericParam::Const(v) => v.span(),
        }
    }
}

impl Parse for GenericParam {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        if matches!(
            stream.curr(),
            Some(moxy_token::TokenTree::Punct(moxy_token::Punctuation::Quote(_)))
        ) {
            return Ok(GenericParam::Lifetime(stream.parse()?));
        }

        let mut fork = stream.fork();
        fork.skip_while::<crate::Attribute>();

        if fork.peek::<Const>() {
            return Ok(GenericParam::Const(Box::new(stream.parse()?)));
        }

        Ok(GenericParam::Type(Box::new(stream.parse()?)))
    }
}

impl ToTokens for GenericParam {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            GenericParam::Lifetime(v) => v.to_tokens(t),
            GenericParam::Type(v) => v.to_tokens(t),
            GenericParam::Const(v) => v.to_tokens(t),
        }
    }
}
