use crate::ast::{AngleArgs, Punctuated, ReturnType, Type};
use crate::parse::{ParseError, ParseStream};
use crate::token::punct::{Comma, Lt, PathSep};
use crate::token::{Delim, Group, ToTokens};
use crate::{Parse, Span, TokenStream, TokenTree};

#[doc = "Parenthesized path arguments (`Fn(A, B) -> C`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ParenthesizedArgs {
    pub span: Span,
    pub inputs: Punctuated<Type, Comma>,
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
        // Turbofish `::<...>` — an optional leading `::` before the `<`.
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

        // Parenthesized args (`Fn(A) -> B`) are only valid in type/trait paths and
        // are consumed there explicitly (see `parse_parenthesized`), never here —
        // in expression position `path(args)` is a call, not path arguments.
        Ok(PathArguments::None)
    }
}

impl PathArguments {
    /// Parse a parenthesized argument list (`(A, B) -> C`) for `Fn`-family paths.
    /// Used by type/bound parsing, not by expression paths.
    pub fn parse_parenthesized(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let group = stream.parse_group(Delim::Paren)?;
        let mut inner = group.parse();
        let inputs = Punctuated::parse_terminated(&mut inner)?;
        let output = stream.parse::<ReturnType>()?;
        Ok(PathArguments::Parenthesized(ParenthesizedArgs {
            span: Span::default(),
            inputs,
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
                let mut inner = TokenStream::new();
                p.inputs.to_tokens(&mut inner);
                tokens.extend_one(TokenTree::Group(Group::new(Delim::Paren, inner)));
                p.output.to_tokens(tokens);
            }
        }
    }
}
