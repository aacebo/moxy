use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use super::{BareFnArg, Variadic};
use crate::*;

/// A parameter of a bare function pointer type: an argument, or a C-style
/// variadic, which rustc's parser accepts in any position, as in
/// `fn(u8, ..., u8)`.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum BareFnParam {
    Arg(BareFnArg),
    Variadic(Variadic),
}

impl Spanner for BareFnParam {
    fn span(&self) -> Span {
        match self {
            Self::Arg(v) => v.span(),
            Self::Variadic(v) => v.span(),
        }
    }
}

impl Parse for BareFnParam {
    fn peek(cursor: Cursor<'_>) -> bool {
        Variadic::peek(cursor) || BareFnArg::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        if Variadic::peek(parser.cursor()) {
            Ok(Self::Variadic(<_ as Parse>::parse(parser)?))
        } else {
            Ok(Self::Arg(<_ as Parse>::parse(parser)?))
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        if Variadic::peek(cursor) {
            Variadic::skip(cursor)
        } else {
            BareFnArg::skip(cursor)
        }
    }
}

impl ToTokens for BareFnParam {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            Self::Arg(v) => v.to_tokens(t),
            Self::Variadic(v) => v.to_tokens(t),
        }
    }
}
