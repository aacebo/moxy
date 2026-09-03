use crate::{Parse, ParseError, Parser};
use moxy_token::Token;
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use super::{Receiver, Variadic};
use crate::pat::PatType;
use crate::{Lifetime, Punctuated};

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct FnParams {
    pub inputs: Punctuated<FnParam, Token![,]>,
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
                <Token![,]>::default().to_tokens(t);
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
    pub fn is_receiver(parser: &Parser) -> bool {
        let fork = parser.lookahead();
        fork.skip_while::<crate::Attribute>();

        if fork.peek::<Token![self]>() {
            return true;
        }

        if fork.peek::<Token![&]>() {
            fork.advance();

            if fork.peek::<Lifetime>() {
                fork.advance();
                fork.advance();
            }

            if fork.peek::<Token![mut]>() {
                fork.advance();
            }

            return fork.peek::<Token![self]>();
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
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        if Self::is_receiver(parser) {
            return Ok(Self::Receiver(Box::new(parser.parse()?)));
        }

        Ok(Self::Typed(Box::new(parser.parse()?)))
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
