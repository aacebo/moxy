use moxy_token::{Ident, Keyword, Span, Spanner, ToTokenStream, ToTokens, TokenStream};

use crate::*;

/// A single segment of a path (an identifier optionally followed by generic arguments).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PathSegment {
    pub ident: Ident,
    pub args: path::PathArguments,
}

impl PathSegment {
    // A segment in expression position takes generic arguments only after `::`
    pub(crate) fn parse_expr(parser: &Parser) -> Result<Self, ParseError> {
        let ident = parser.parse_ident_any()?;
        let args = if Self::peek_turbofish(parser.cursor()) {
            AngleArguments::parse(parser)?.into()
        } else {
            path::PathArguments::None
        };

        Ok(Self { ident, args })
    }

    pub(crate) fn skip_expr(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        let cursor = if Ident::peek(cursor) {
            Ident::skip(cursor)?
        } else {
            Keyword::skip(cursor)?
        };

        if Self::peek_turbofish(cursor) {
            AngleArguments::skip(cursor)
        } else {
            Some(cursor)
        }
    }

    fn peek_turbofish(cursor: Cursor<'_>) -> bool {
        <Token![::]>::skip(cursor).is_some_and(|cursor| <Token![<]>::peek(cursor))
    }
}

impl Parse for PathSegment {
    fn peek(cursor: Cursor<'_>) -> bool {
        Keyword::peek(cursor) || Ident::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        // Any segment of a type path can take either form of arguments, as in
        // `Fn<T>`, `AsyncFn() -> u8`, and `MyTrait(A) -> B`
        Ok(Self {
            ident: parser.parse_ident_any()?,
            args: <_ as Parse>::parse(parser)?,
        })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = if Ident::peek(cursor) {
            Ident::skip(cursor)?
        } else {
            Keyword::skip(cursor)?
        };

        PathArguments::skip(cursor)
    }
}

impl Spanner for PathSegment {
    fn span(&self) -> Span {
        match &self.args {
            PathArguments::None => self.ident.span(),
            args => self.ident.span().join(args.span()),
        }
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
