use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A glob import (`*`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct UseGlob {
    pub star: Token![*],
}

impl Parse for UseGlob {
    fn peek(cursor: Cursor<'_>) -> bool {
        <Token![*]>::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            star: <_ as Parse>::parse(parser)?,
        })
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        <Token![*]>::skip(cursor)
    }
}

impl Spanner for UseGlob {
    fn span(&self) -> Span {
        self.star.span()
    }
}

impl ToTokens for UseGlob {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.star.to_tokens(t);
    }
}
