use moxy_token::{Ident, Span, Spanner, ToTokenStream, ToTokens, TokenStream};

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
        let ident = Self::parse_name(parser)?;
        let args = if Self::peek_turbofish(parser.cursor()) {
            AngleArguments::parse(parser)?.into()
        } else {
            path::PathArguments::None
        };

        Ok(Self { ident, args })
    }

    pub(crate) fn skip_expr(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        let cursor = Self::peek_name(cursor).then(|| cursor.offset(1))?;

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

impl PathSegment {
    // Whether a segment's name starts here: an identifier, a path keyword such
    // as `self` or `crate`, or a weak keyword such as `union`, which is an
    // identifier outside its own syntax. A strict keyword such as `as` names
    // no segment.
    pub(crate) fn peek_name(cursor: Cursor<'_>) -> bool {
        Ident::peek(cursor)
            || cursor.curr().is_some_and(|token| {
                token.is_keyword_crate()
                    || token.is_keyword_self_value()
                    || token.is_keyword_self_type()
                    || token.is_keyword_super()
                    || token.is_keyword_auto()
                    || token.is_keyword_default()
                    || token.is_keyword_macro_rules()
                    || token.is_keyword_raw()
                    || token.is_keyword_safe()
                    || token.is_keyword_union()
            })
    }
}

impl PathSegment {
    fn parse_name(parser: &Parser) -> Result<Ident, ParseError> {
        if !Self::peek_name(parser.cursor()) {
            return parser.error("expected identifier").into();
        }

        parser.parse_ident_any()
    }
}

impl Parse for PathSegment {
    fn peek(cursor: Cursor<'_>) -> bool {
        Self::peek_name(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        // Any segment of a type path can take either form of arguments, as in
        // `Fn<T>`, `AsyncFn() -> u8`, and `MyTrait(A) -> B`
        Ok(Self {
            ident: Self::parse_name(parser)?,
            args: <_ as Parse>::parse(parser)?,
        })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = Self::peek_name(cursor).then(|| cursor.offset(1))?;
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
