use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A leaf name in a use tree (`foo`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct UseName {
    pub ident: Ident,
}

impl Parse for UseName {
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.peek::<Ident>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self { ident: parser.parse()? })
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor.skip::<Ident>()
    }
}

impl Spanner for UseName {
    fn span(&self) -> Span {
        self.ident.span()
    }
}

impl ToTokens for UseName {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.ident.to_tokens(t);
    }
}
