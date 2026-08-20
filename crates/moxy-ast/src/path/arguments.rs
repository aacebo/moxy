use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::{Lt, PathSep};
use moxy_token::{Parse, Span, Spanner, ToTokens, TokenStream};

use crate::{AngleArguments, ParenArguments};

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
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let mut fork = stream.fork();

        if fork.peek::<PathSep>() {
            let _ = fork.parse::<PathSep>()?;

            if fork.peek::<Lt>() {
                stream.seek(&fork);
            }
        }

        if stream.peek::<Lt>() {
            return Ok(Self::AngleBracketed(stream.parse()?));
        }

        Ok(Self::None)
    }
}

impl PathArguments {
    pub fn parse_parenthesized(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let args = stream.parse::<ParenArguments>()?;
        Ok(Self::Parenthesized(args))
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
