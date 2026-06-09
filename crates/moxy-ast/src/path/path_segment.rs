use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::{Delim, Parse, Span, Spanner, ToTokens, TokenStream, TokenTree};

use super::PathArguments;
use crate::Ident;

/// A single segment of a path (an identifier optionally followed by generic arguments).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PathSegment {
    pub ident: Ident,
    pub args: PathArguments,
}

impl PathSegment {
    pub fn is_fn_family(ident: &Ident) -> bool {
        matches!(ident.text(), "Fn" | "FnMut" | "FnOnce")
    }
}

impl Parse for PathSegment {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let ident = stream.parse::<Ident>()?;

        // `Fn`-family segments take parenthesized args (`Fn(A) -> B`); this only
        // applies to those trait names, so it never swallows expression calls.
        let args = if PathSegment::is_fn_family(&ident)
            && matches!(stream.curr(), Some(TokenTree::Group(g)) if g.delim() == Delim::Paren)
        {
            PathArguments::parse_parenthesized(stream)?
        } else {
            stream.parse::<PathArguments>()?
        };

        Ok(Self { ident, args })
    }
}

impl Spanner for PathSegment {
    fn span(&self) -> Span {
        self.ident.span()
    }
}

impl ToTokens for PathSegment {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.ident.to_tokens(tokens);
        self.args.to_tokens(tokens);
    }
}
