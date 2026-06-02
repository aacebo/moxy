use super::{ConstParam, LifetimeParam, TypeParam};
use crate::parse::{ParseError, ParseStream};
use crate::token::ToTokens;
use crate::token::keyword::Const;
use crate::{Parse, TokenStream};

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
            Some(crate::TokenTree::Token(crate::Token::Punct(
                crate::token::Punctuation::Quote(_)
            )))
        ) {
            return Ok(GenericParam::Lifetime(stream.parse()?));
        }

        let mut fork = stream.fork();
        let _ = fork.parse_vec::<crate::ast::Attribute>();

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
