use moxy_token::{Ident, Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A renamed use leaf (`foo as bar`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct UseRename {
    pub ident: Ident,
    pub as_keyword: Token![as],
    pub rename: Ident,
}

impl Parse for UseRename {
    fn peek(cursor: Cursor<'_>) -> bool {
        Ident::peek(cursor) && <Token![as]>::peek(cursor.offset(1)) && Ident::peek(cursor.offset(2))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            ident: <_ as Parse>::parse(parser)?,
            as_keyword: <_ as Parse>::parse(parser)?,
            rename: <_ as Parse>::parse(parser)?,
        })
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Ident::skip(<Token![as]>::skip(Ident::skip(cursor)?)?)
    }
}

impl Spanner for UseRename {
    fn span(&self) -> Span {
        self.ident.span().join(self.rename.span())
    }
}

impl ToTokens for UseRename {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.ident.to_tokens(t);
        self.as_keyword.to_tokens(t);
        self.rename.to_tokens(t);
    }
}
