use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use super::{Receiver, Variadic};
use crate::*;

/// An AST representation of Rust fn params syntax.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct FnParams {
    pub inputs: List<FnParam, Token![,]>,
}

impl Spanner for FnParams {
    fn span(&self) -> Span {
        let start = self.inputs.first().map(|i| i.span()).unwrap_or_else(Span::call_site);
        let end = self.inputs.last().map(|i| i.span()).unwrap_or_else(Span::call_site);
        start.join(end)
    }
}

impl ToTokens for FnParams {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.inputs.to_tokens(t);
    }
}

/// A function parameter: a receiver, a typed pattern, a type alone, or a
/// C-style variadic, which rustc's parser accepts in any position, as in
/// `fn f(a: u8, ..., b: u8);`.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum FnParam {
    Receiver(Box<Receiver>),
    Typed(Box<pat::PatType>),
    Type(Box<Type>),
    Variadic(Box<Variadic>),
}

impl Spanner for FnParam {
    fn span(&self) -> Span {
        match self {
            Self::Receiver(v) => v.span(),
            Self::Typed(v) => v.span(),
            Self::Type(v) => v.span(),
            Self::Variadic(v) => v.span(),
        }
    }
}

impl Parse for FnParam {
    fn peek(cursor: Cursor<'_>) -> bool {
        Variadic::peek(cursor) || Receiver::peek(cursor) || pat::PatType::peek(cursor) || Type::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        if Variadic::peek(parser.cursor()) {
            return Ok(Self::Variadic(Box::new(<_ as Parse>::parse(parser)?)));
        }

        if Receiver::peek(parser.cursor()) {
            return Ok(Self::Receiver(Box::new(<_ as Parse>::parse(parser)?)));
        }

        // As `skip` reads it: a typed pattern, or else a type. A pattern that
        // fails to peek for the depth limit, where no type reads either, parses
        // as a typed pattern so that its parse reports the depth error
        let watch = moxy_token::__private::Watch::start();
        let typed = pat::PatType::peek(parser.cursor());

        if typed || watch.exceeded() && Type::skip(parser.cursor()).is_none() {
            Ok(Self::Typed(Box::new(<_ as Parse>::parse(parser)?)))
        } else {
            Ok(Self::Type(Box::new(<_ as Parse>::parse(parser)?)))
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::skip_named(cursor).or_else(|| Type::skip(cursor))
    }
}

impl FnParam {
    // Skips a receiver, a variadic, or a parameter with a pattern, not a type
    // alone
    pub(crate) fn skip_named(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        if Variadic::peek(cursor) {
            Variadic::skip(cursor)
        } else if Receiver::peek(cursor) {
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
            Self::Type(v) => v.to_tokens(t),
            Self::Variadic(v) => v.to_tokens(t),
        }
    }
}
