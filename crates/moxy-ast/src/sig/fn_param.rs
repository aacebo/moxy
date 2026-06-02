use moxy_token::parse::{ParseError, ParseStream};
use moxy_token::token::ToTokens;
use moxy_token::token::keyword::SelfValue;
use moxy_token::token::punct::And;
use moxy_token::{Parse, TokenStream};

use super::Receiver;
use crate::{Lifetime, Mutability, TypedParam};

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
        let _ = fork.parse_vec::<crate::Attribute>();

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
