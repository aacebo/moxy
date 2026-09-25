use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// The limits of a range expression (`..` or `..=`).
#[derive(Copy, Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum RangeLimits {
    Closed(Token![..=]),
    HalfOpen(Token![..]),
}

impl Spanner for RangeLimits {
    fn span(&self) -> Span {
        match self {
            Self::Closed(v) => v.span(),
            Self::HalfOpen(v) => v.span(),
        }
    }
}

impl ToTokens for RangeLimits {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Closed(v) => v.to_tokens(tokens),
            Self::HalfOpen(v) => v.to_tokens(tokens),
        }
    }
}

impl Parse for RangeLimits {
    fn peek(cursor: Cursor<'_>) -> bool {
        <Token![..=]>::peek(cursor) || <Token![..]>::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        if <Token![..=]>::peek(parser.cursor()) {
            Ok(Self::Closed(<_ as Parse>::parse(parser)?))
        } else {
            Ok(Self::HalfOpen(<_ as Parse>::parse(parser)?))
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        if <Token![..=]>::peek(cursor) {
            <Token![..=]>::skip(cursor)
        } else {
            <Token![..]>::skip(cursor)
        }
    }
}

impl std::fmt::Display for RangeLimits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Closed(v) => write!(f, "{v}"),
            Self::HalfOpen(v) => write!(f, "{v}"),
        }
    }
}
