use moxy_token::keyword::SelfValue;
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::And;
use moxy_token::{Comma, Parse, Span, Spanner, ToTokens, TokenStream};

use super::{Receiver, Variadic};
use crate::pat::PatType;
use crate::{Lifetime, Mutability, Punctuated};

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct FnParams {
    pub inputs: Punctuated<FnParam, Comma>,
    pub variadic: Option<Variadic>,
}

impl Spanner for FnParams {
    fn span(&self) -> Span {
        let start = self.inputs.first().map(|i| i.span()).unwrap_or_else(Span::call_site);
        let end = self
            .variadic
            .as_ref()
            .map(|v| v.span())
            .or_else(|| self.inputs.last().map(|i| i.span()))
            .unwrap_or_else(Span::call_site);
        start.join(end)
    }
}

impl ToTokens for FnParams {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.inputs.to_tokens(t);
        if let Some(v) = &self.variadic {
            if !self.inputs.is_empty() && !self.inputs.is_trailing() {
                Comma::default().to_tokens(t);
            }
            v.to_tokens(t);
        }
    }
}

/// A function parameter (receiver or typed pattern).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum FnParam {
    Receiver(Box<Receiver>),
    Typed(Box<PatType>),
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
