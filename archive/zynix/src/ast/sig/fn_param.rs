use super::Receiver;
use crate::ast::{Lifetime, Mutability, TypedParam};
use crate::parse::{ParseError, ParseStream};
use crate::token::ToTokens;
use crate::token::keyword::SelfValue;
use crate::token::punct::And;
use crate::{Parse, TokenStream};

#[doc = "A function parameter (receiver or typed pattern)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum FnParam {
    Receiver(Box<Receiver>),
    Typed(Box<TypedParam>),
}

impl FnParam {
    pub fn is_receiver(stream: &mut ParseStream) -> bool {
        let mut fork = stream.fork();
        let _ = fork.parse_vec::<crate::ast::Attribute>();

        if fork.peek::<SelfValue>().is_some() {
            return true;
        }

        if fork.peek::<And>().is_some() {
            let _ = fork.parse::<And>();
            let _ = fork.parse_opt::<Lifetime>();
            let _ = fork.parse::<Mutability>();
            return fork.peek::<SelfValue>().is_some();
        }

        false
    }
}

impl Parse for FnParam {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        if FnParam::is_receiver(stream) {
            return Ok(FnParam::Receiver(Box::new(stream.parse()?)));
        }

        Ok(FnParam::Typed(Box::new(stream.parse()?)))
    }
}

impl ToTokens for FnParam {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            FnParam::Receiver(v) => v.to_tokens(t),
            FnParam::Typed(v) => v.to_tokens(t),
        }
    }
}
