use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A literal pattern, e.g. `42`, `'a'`, or `"hello"`.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatLit {
    pub attrs: Attributes,
    pub lit: Lit,
}

impl Spanner for PatLit {
    fn span(&self) -> Span {
        self.attrs.span().join(self.lit.span())
    }
}

impl Parse for PatLit {
    fn peek(cursor: Cursor<'_>) -> bool {
        let cursor = Attributes::skip(cursor).unwrap_or(cursor);
        Lit::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            attrs: <_ as Parse>::parse(parser)?,
            lit: <_ as Parse>::parse(parser)?,
        })
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Lit::skip(Attributes::skip(cursor)?)
    }
}

impl ToTokens for PatLit {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.lit.to_tokens(t);
    }
}
