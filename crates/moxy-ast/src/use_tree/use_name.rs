use moxy_token::{Ident, Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A leaf name in a use tree (`foo`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct UseName {
    /// The leading `::` of a path, as in `use ::serde;`.
    pub prefix: Option<Token![::]>,
    pub ident: Ident,
}

impl Parse for UseName {
    fn peek(cursor: Cursor<'_>) -> bool {
        super::peek_segment(super::skip_prefix(cursor))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            prefix: <_ as Parse>::parse(parser)?,
            ident: parser.parse_ident_any()?,
        })
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        super::skip_segment(super::skip_prefix(cursor))
    }
}

impl Spanner for UseName {
    fn span(&self) -> Span {
        match self.prefix {
            Some(prefix) => prefix.span().join(self.ident.span()),
            None => self.ident.span(),
        }
    }
}

impl ToTokens for UseName {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.prefix.to_tokens(t);
        self.ident.to_tokens(t);
    }
}
