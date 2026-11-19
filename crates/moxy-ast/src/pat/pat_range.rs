use moxy_token::{Ident, Lit, Span, Spanner, ToTokenStream, ToTokens, TokenStream, TokenTree};

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

impl Attributed for PatRange {
    fn attrs(&self) -> &[Attribute] {
        &self.attrs
    }
}

impl Spanner for PatRange {
    fn span(&self) -> Span {
        self.to_token_stream().span()
    }
}

impl Parse for PatRange {
    // A bound is a literal, a negated literal, a path, or a block, so only a
    // token that starts one, or the limits, starts a range. Without that check
    // every reference pattern skipped the rest of the input as an expression.
    fn peek(cursor: Cursor<'_>) -> bool {
        let start = Attributes::skip(cursor).unwrap_or(cursor);
        let bound = PatRangeLimits::peek(start)
            || Lit::peek(start)
            || <Token![-]>::peek(start)
            || <Token![<]>::peek(start)
            || <Token![const]>::peek(start)
            || Path::peek(start);

        bound && Self::skip(cursor).is_some()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = <_ as Parse>::parse(parser)?;
        let start = if PatRangeLimits::peek(parser.cursor()) {
            None
        } else {
            Some(expr::parse::unary(parser, Attributes::default())?)
        };

        let limits = <_ as Parse>::parse(parser)?;
        let end = if Self::peek_end(parser.cursor()) {
            Some(expr::parse::unary(parser, Attributes::default())?)
        } else {
            None
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

        if !Self::peek_end(cursor) {
            return (has_start || closed).then_some(cursor);
        }

        expr::skip::pattern_bound(cursor)
    }
}

impl PatRange {
    // A range bound is a literal, a negated literal, or a path, so any other
    // token, such as the `=>` or `if` of a match arm, ends a half-open `a..`
    fn peek_end(cursor: Cursor<'_>) -> bool {
        cursor.curr().is_some_and(|token| {
            matches!(token, TokenTree::Literal(_))
                || token.is_keyword_crate()
                || token.is_keyword_self_value()
                || token.is_keyword_self_type()
                || token.is_keyword_super()
        }) || Ident::peek(cursor)
            || <Token![-]>::peek(cursor)
            || <Token![::]>::peek(cursor)
            || <Token![<]>::peek(cursor)
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
