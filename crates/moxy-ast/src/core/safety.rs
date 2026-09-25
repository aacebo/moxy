use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// An explicit item safety modifier (`safe` or `unsafe`).
#[derive(Copy, Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Safety {
    Safe(Token![safe]),
    Unsafe(Token![unsafe]),
}

impl Parse for Safety {
    fn peek(cursor: Cursor<'_>) -> bool {
        <Token![safe]>::peek(cursor) || <Token![unsafe]>::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        if <Token![safe]>::peek(parser.cursor()) {
            Ok(Self::Safe(<_ as Parse>::parse(parser)?))
        } else {
            Ok(Self::Unsafe(<_ as Parse>::parse(parser)?))
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        if <Token![safe]>::peek(cursor) {
            <Token![safe]>::skip(cursor)
        } else {
            <Token![unsafe]>::skip(cursor)
        }
    }
}

impl Spanner for Safety {
    fn span(&self) -> Span {
        match self {
            Self::Safe(v) => v.span(),
            Self::Unsafe(v) => v.span(),
        }
    }
}

impl ToTokens for Safety {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Safe(v) => v.to_tokens(tokens),
            Self::Unsafe(v) => v.to_tokens(tokens),
        }
    }
}
