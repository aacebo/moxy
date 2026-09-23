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
        cursor.peek::<Token![..=]>() || cursor.peek::<Token![..]>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        if parser.peek::<Token![..=]>() {
            Ok(Self::Closed(parser.parse()?))
        } else {
            Ok(Self::HalfOpen(parser.parse()?))
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        if cursor.peek::<Token![..=]>() {
            cursor.skip::<Token![..=]>()
        } else {
            cursor.skip::<Token![..]>()
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
