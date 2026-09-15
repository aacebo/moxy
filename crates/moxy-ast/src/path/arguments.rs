use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// The arguments of a path segment: none, angle-bracketed (`<T>`), or parenthesized (`Fn(A) -> B`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum PathArguments {
    None,
    AngleBracketed(AngleArguments),
    Parenthesized(ParenArguments),
}

impl From<AngleArguments> for PathArguments {
    fn from(v: AngleArguments) -> Self {
        Self::AngleBracketed(v)
    }
}

impl From<ParenArguments> for PathArguments {
    fn from(v: ParenArguments) -> Self {
        Self::Parenthesized(v)
    }
}

impl Parse for PathArguments {
    fn peek(cursor: Cursor<'_>) -> bool {
        true
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        if parser.peek::<AngleArguments>() {
            Ok(Self::AngleBracketed(parser.parse()?))
        } else if parser.peek::<ParenArguments>() {
            Ok(Self::Parenthesized(parser.parse()?))
        } else {
            Ok(Self::None)
        }
    }
}

impl Spanner for PathArguments {
    fn span(&self) -> Span {
        match self {
            Self::None => Span::call_site(),
            Self::AngleBracketed(v) => v.span(),
            Self::Parenthesized(v) => v.span(),
        }
    }
}

impl ToTokens for PathArguments {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::None => {}
            Self::AngleBracketed(args) => args.to_tokens(tokens),
            Self::Parenthesized(args) => args.to_tokens(tokens),
        }
    }
}
