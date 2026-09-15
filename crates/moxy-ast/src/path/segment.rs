use moxy_token::{Keyword, Span, Spanner, ToTokenStream, ToTokens, TokenStream};

use crate::*;

/// A single segment of a path (an identifier optionally followed by generic arguments).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PathSegment {
    pub ident: Ident,
    pub args: path::PathArguments,
}

impl Parse for PathSegment {
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.peek::<Keyword>() || cursor.peek::<Ident>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let ident = parser.parse_ident_any()?;
        let args = if matches!(ident.text(), "Fn" | "FnMut" | "FnOnce") {
            path::PathArguments::Parenthesized(parser.parse()?)
        } else {
            parser.parse()?
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
