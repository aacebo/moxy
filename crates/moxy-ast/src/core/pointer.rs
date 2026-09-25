use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// Whether a raw pointer is `*const` or `*mut`.
#[derive(Copy, Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum PointerMutability {
    Const(Token![const]),
    Mut(Token![mut]),
}

impl Spanner for PointerMutability {
    fn span(&self) -> Span {
        match self {
            Self::Const(v) => v.span(),
            Self::Mut(v) => v.span(),
        }
    }
}

impl ToTokens for PointerMutability {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Const(v) => v.to_tokens(tokens),
            Self::Mut(v) => v.to_tokens(tokens),
        }
    }
}

impl Parse for PointerMutability {
    fn peek(cursor: Cursor<'_>) -> bool {
        <Token![const]>::peek(cursor) || <Token![mut]>::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        if <Token![const]>::peek(parser.cursor()) {
            Ok(Self::Const(parser.parse()?))
        } else {
            Ok(Self::Mut(parser.parse()?))
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        if <Token![const]>::peek(cursor) {
            <Token![const]>::skip(cursor)
        } else {
            <Token![mut]>::skip(cursor)
        }
    }
}

impl std::fmt::Display for PointerMutability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Const(v) => write!(f, "{v}"),
            Self::Mut(v) => write!(f, "{v}"),
        }
    }
}
