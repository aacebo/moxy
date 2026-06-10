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
        PathArguments::AngleBracketed(v)
    }
}

impl From<ParenArguments> for PathArguments {
    fn from(v: ParenArguments) -> Self {
        PathArguments::Parenthesized(v)
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
            return Ok(PathArguments::AngleBracketed(stream.parse()?));
        }

        Ok(PathArguments::None)
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
            PathArguments::None => Span::call_site(),
            PathArguments::AngleBracketed(v) => v.span(),
            PathArguments::Parenthesized(v) => v.span(),
        }
    }
}

impl ToTokens for PathArguments {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            PathArguments::None => {}
            PathArguments::AngleBracketed(args) => args.to_tokens(tokens),
            PathArguments::Parenthesized(args) => args.to_tokens(tokens),
        }
    }
}
