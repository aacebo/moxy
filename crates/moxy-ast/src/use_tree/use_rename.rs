use moxy_token::{Ident, Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A renamed use leaf (`foo as bar`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct UseRename {
    /// The leading `::` of a path, as in `use ::core as core2;`.
    pub prefix: Option<Token![::]>,
    pub ident: Ident,
    pub as_keyword: Token![as],
    pub rename: Ident,
}

impl Parse for UseRename {
    fn peek(cursor: Cursor<'_>) -> bool {
        let cursor = super::skip_prefix(cursor);
        super::peek_segment(cursor) && <Token![as]>::peek(cursor.offset(1)) && Ident::peek(cursor.offset(2))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            prefix: <_ as Parse>::parse(parser)?,
            ident: parser.parse_ident_any()?,
            as_keyword: <_ as Parse>::parse(parser)?,
            rename: <_ as Parse>::parse(parser)?,
        })
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Ident::skip(<Token![as]>::skip(super::skip_segment(super::skip_prefix(cursor))?)?)
    }
}

impl Spanner for UseRename {
    fn span(&self) -> Span {
        let start = self.prefix.map(|prefix| prefix.span()).unwrap_or_else(|| self.ident.span());
        start.join(self.rename.span())
    }
}

impl ToTokens for UseRename {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.prefix.to_tokens(t);
        self.ident.to_tokens(t);
        self.as_keyword.to_tokens(t);
        self.rename.to_tokens(t);
    }
}
