use moxy_token::{Keyword, Span, Spanner, ToTokenStream, ToTokens, TokenStream};

use crate::*;

/// A single segment of a path (an identifier optionally followed by generic arguments).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
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
        let is_fn = matches!(ident.text(), "Fn" | "FnMut" | "FnOnce");

        Ok(Self {
            ident,
            args: if is_fn && parser.peek::<ParenArguments>() {
                parser.parse::<ParenArguments>()?.into()
            } else if !is_fn && parser.peek::<AngleArguments>() {
                parser.parse::<AngleArguments>()?.into()
            } else {
                path::PathArguments::None
            },
        })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        let is_fn = matches!(cursor.curr().and_then(|token| token.text()), Some("Fn" | "FnMut" | "FnOnce"));

        cursor = if cursor.peek::<Ident>() {
            cursor.skip::<Ident>()?
        } else {
            cursor.skip::<Keyword>()?
        };

        if is_fn && cursor.peek::<ParenArguments>() {
            cursor.skip::<ParenArguments>()
        } else {
            cursor.skip::<Option<AngleArguments>>()
        }
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
