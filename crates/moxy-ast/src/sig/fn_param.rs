use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use super::{Receiver, Variadic};
use crate::*;

/// An AST representation of Rust fn params syntax.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
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
        self.variadic.to_tokens(t);
    }
}

/// A function parameter (receiver or typed pattern).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum FnParam {
    Receiver(Box<Receiver>),
    Typed(Box<pat::PatType>),
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
    fn peek(cursor: Cursor<'_>) -> bool {
        Receiver::peek(cursor) || pat::PatType::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        if Receiver::peek(parser.cursor()) {
            return Ok(Self::Receiver(Box::new(<_ as Parse>::parse(parser)?)));
        }

        Ok(Self::Typed(Box::new(<_ as Parse>::parse(parser)?)))
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        if Receiver::peek(cursor) {
            Receiver::skip(cursor)
        } else {
            pat::PatType::skip(cursor)
        }
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
