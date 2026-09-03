use crate::{Parse, ParseError, Parser};
use moxy_token::{Delim, Span, Spanner, ToTokenStream, ToTokens, TokenStream, TokenTree};

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
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let ident = parser.parse_ident_any()?;

        // `Fn`-family segments take parenthesized args (`Fn(A) -> B`); this only
        // applies to those trait names, so it never swallows expression calls.
        let args =
            if Self::is_fn_family(&ident) && matches!(parser.curr(), Some(TokenTree::Group(g)) if g.delim() == Delim::Paren) {
                PathArguments::parse_parenthesized(parser)?
            } else {
                parser.parse::<PathArguments>()?
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

impl std::hash::Hash for PathSegment {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.to_token_stream().to_string().hash(state);
    }
}
