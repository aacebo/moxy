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
        Keyword::peek(cursor) || Ident::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let ident = parser.parse_ident_any()?;
        let is_fn = matches!(ident.text(), "Fn" | "FnMut" | "FnOnce");

        Ok(Self {
            ident,
            args: if is_fn && ParenArguments::peek(parser.cursor()) {
                ParenArguments::parse(parser)?.into()
            } else if !is_fn && AngleArguments::peek(parser.cursor()) {
                AngleArguments::parse(parser)?.into()
            } else {
                path::PathArguments::None
            },
        })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        let is_fn = matches!(cursor.curr().and_then(|token| token.text()), Some("Fn" | "FnMut" | "FnOnce"));

        cursor = if Ident::peek(cursor) {
            Ident::skip(cursor)?
        } else {
            Keyword::skip(cursor)?
        };

        if is_fn && ParenArguments::peek(cursor) {
            ParenArguments::skip(cursor)
        } else {
            Option::<AngleArguments>::skip(cursor)
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
