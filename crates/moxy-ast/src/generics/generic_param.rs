use moxy_token::keyword::Const;
use moxy_token::parse::{ParseError, ParseStream};
use moxy_token::{Parse, ToTokens, TokenStream};

use super::{ConstParam, LifetimeParam, TypeParam};

#[doc = "A generic parameter (lifetime, type, or const)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum GenericParam {
    Lifetime(LifetimeParam),
    Type(TypeParam),
    Const(ConstParam),
}

impl From<LifetimeParam> for GenericParam {
    fn from(v: LifetimeParam) -> Self {
        GenericParam::Lifetime(v)
    }
}

impl From<TypeParam> for GenericParam {
    fn from(v: TypeParam) -> Self {
        GenericParam::Type(v)
    }
}

impl From<ConstParam> for GenericParam {
    fn from(v: ConstParam) -> Self {
        GenericParam::Const(v)
    }
}

impl Parse for GenericParam {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        if matches!(
            stream.curr(),
            Some(moxy_token::TokenTree::Token(moxy_token::Token::Punct(
                moxy_token::Punctuation::Quote(_)
            )))
        ) {
            return Ok(GenericParam::Lifetime(stream.parse()?));
        }

        let mut fork = stream.fork();
        let _ = fork.parse_vec::<crate::Attribute>();

        if fork.peek::<Const>().is_some() {
            return Ok(GenericParam::Const(stream.parse()?));
        }

        Ok(GenericParam::Type(stream.parse()?))
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
