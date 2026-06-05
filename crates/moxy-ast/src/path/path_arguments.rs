use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::{Comma, Lt, PathSep};
use moxy_token::{Parse, Span, ToTokens, TokenStream};

use crate::{AngleArgs, Delimited, Punctuated, ReturnType, Type};

#[doc = "Parenthesized path arguments (`Fn(A, B) -> C`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ParenthesizedArgs {
    pub span: Span,
    pub params: Delimited<Punctuated<Type, Comma>>,
    pub output: ReturnType,
}

#[doc = "The arguments of a path segment: none, angle-bracketed (`<T>`), or parenthesized (`Fn(A) -> B`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum PathArguments {
    None,
    AngleBracketed(AngleArgs),
    Parenthesized(ParenthesizedArgs),
}

impl From<AngleArgs> for PathArguments {
    fn from(v: AngleArgs) -> Self {
        PathArguments::AngleBracketed(v)
    }
}

impl From<ParenthesizedArgs> for PathArguments {
    fn from(v: ParenthesizedArgs) -> Self {
        PathArguments::Parenthesized(v)
    }
}

impl Parse for PathArguments {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let mut fork = stream.fork();
        if fork.peek::<PathSep>().is_some() {
            let _ = fork.parse::<PathSep>()?;
            if fork.peek::<Lt>().is_some() {
                stream.seek(&fork);
            }
        }

        if stream.peek::<Lt>().is_some() {
            return Ok(PathArguments::AngleBracketed(stream.parse()?));
        }

        Ok(PathArguments::None)
    }
}

impl PathArguments {
    pub fn parse_parenthesized(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let params = Delimited::parse_paren_with(stream, Punctuated::parse_terminated)?;
        let output = stream.parse::<ReturnType>()?;
        Ok(PathArguments::Parenthesized(ParenthesizedArgs {
            span: Span::default(),
            params,
            output,
        }))
    }
}

impl ToTokens for PathArguments {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            PathArguments::None => {}
            PathArguments::AngleBracketed(args) => args.to_tokens(tokens),
            PathArguments::Parenthesized(p) => {
                p.params.to_tokens(tokens);
                p.output.to_tokens(tokens);
            }
        }
    }
}
