use moxy_token::keyword::SelfValue;
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::And;
use moxy_token::{Parse, Span, Spanner, ToTokens, TokenStream};

use super::Receiver;
use crate::{Lifetime, Mutability, TypedParam};

/// A function parameter (receiver or typed pattern).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum FnParam {
    Receiver(Box<Receiver>),
    Typed(Box<TypedParam>),
}

impl FnParam {
    pub fn is_receiver(stream: &mut ParseStream) -> bool {
        let mut fork = stream.fork();
        fork.skip_while::<crate::Attribute>();

        if fork.peek::<SelfValue>() {
            return true;
        }

        if fork.peek::<And>() {
            let _ = fork.parse::<And>();
            let _ = fork.parse_if::<Lifetime>();
            let _ = fork.parse::<Mutability>();
            return fork.peek::<SelfValue>();
        }

        false
    }
}

impl Spanner for FnParam {
    fn span(&self) -> Span {
        match self {
            Self::Receiver(v) => v.span(),
            Self::Typed(v) => v.span(),
        }
    }
}

impl Parse for FnParam {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        if Self::is_receiver(stream) {
            return Ok(Self::Receiver(Box::new(stream.parse()?)));
        }

        Ok(Self::Typed(Box::new(stream.parse()?)))
    }
}

impl ToTokens for FnParam {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            Self::Receiver(v) => v.to_tokens(t),
            Self::Typed(v) => v.to_tokens(t),
        }
    }
}
