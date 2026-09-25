use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A range pattern, e.g. `0..=255` or `'a'..'z'`.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatRange {
    pub attrs: Attributes,
    pub start: Option<Expr>,
    pub limits: PatRangeLimits,
    pub end: Option<Expr>,
}

impl Spanner for PatRange {
    fn span(&self) -> Span {
        let end = if let Some(e) = &self.end {
            e.span()
        } else {
            self.limits.span()
        };

        self.attrs.span().join(end)
    }
}

impl Parse for PatRange {
    fn peek(cursor: Cursor<'_>) -> bool {
        Self::skip(cursor).is_some()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = <_ as Parse>::parse(parser)?;
        let start = if PatRangeLimits::peek(parser.cursor()) {
            None
        } else {
            Some(expr::parse::unary(parser, Attributes::default())?)
        };

        let limits = <_ as Parse>::parse(parser)?;
        let end = if parser.is_empty()
            || <Token![,]>::peek(parser.cursor())
            || <Token![|]>::peek(parser.cursor())
            || <Token![:]>::peek(parser.cursor())
        {
            None
        } else {
            Some(expr::parse::unary(parser, Attributes::default())?)
        };

        if start.is_none() && end.is_none() && matches!(limits, PatRangeLimits::HalfOpen(_)) {
            return parser.error("expected range pattern").into();
        }

        Ok(Self {
            attrs,
            start,
            limits,
            end,
        })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = Attributes::skip(cursor)?;
        let has_start = !PatRangeLimits::peek(cursor);

        if has_start {
            cursor = expr::skip::pattern_bound(cursor)?;
        }

        let closed = <Token![..=]>::peek(cursor);
        cursor = PatRangeLimits::skip(cursor)?;

        if cursor.is_empty() || <Token![,]>::peek(cursor) || <Token![|]>::peek(cursor) || <Token![:]>::peek(cursor) {
            return (has_start || closed).then_some(cursor);
        }

        expr::skip::pattern_bound(cursor)
    }
}

/// The limits of a range pattern, including the obsolete `...` spelling.
#[derive(Copy, Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum PatRangeLimits {
    Closed(Token![..=]),
    HalfOpen(Token![..]),
    Obsolete(Token![...]),
}

impl Parse for PatRangeLimits {
    fn peek(cursor: Cursor<'_>) -> bool {
        <Token![..=]>::peek(cursor) || <Token![...]>::peek(cursor) || <Token![..]>::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        if <Token![..=]>::peek(parser.cursor()) {
            Ok(Self::Closed(<_ as Parse>::parse(parser)?))
        } else if <Token![...]>::peek(parser.cursor()) {
            Ok(Self::Obsolete(<_ as Parse>::parse(parser)?))
        } else {
            Ok(Self::HalfOpen(<_ as Parse>::parse(parser)?))
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        if <Token![..=]>::peek(cursor) {
            <Token![..=]>::skip(cursor)
        } else if <Token![...]>::peek(cursor) {
            <Token![...]>::skip(cursor)
        } else {
            <Token![..]>::skip(cursor)
        }
    }
}

impl Spanner for PatRangeLimits {
    fn span(&self) -> Span {
        match self {
            Self::Closed(v) => v.span(),
            Self::HalfOpen(v) => v.span(),
            Self::Obsolete(v) => v.span(),
        }
    }
}

impl ToTokens for PatRangeLimits {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Closed(v) => v.to_tokens(tokens),
            Self::HalfOpen(v) => v.to_tokens(tokens),
            Self::Obsolete(v) => v.to_tokens(tokens),
        }
    }
}

impl ToTokens for PatRange {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.start.to_tokens(t);
        self.limits.to_tokens(t);
        self.end.to_tokens(t);
    }
}
