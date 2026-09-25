use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A reference pattern, e.g. `&x` or `&mut x`.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatReference {
    pub attrs: Attributes,
    pub and: Token![&],
    pub mutability: Option<Token![mut]>,
    pub pat: Box<Pattern>,
}

impl Spanner for PatReference {
    fn span(&self) -> Span {
        self.attrs.span().join(self.pat.span())
    }
}

impl Parse for PatReference {
    fn peek(cursor: Cursor<'_>) -> bool {
        let cursor = Attributes::skip(cursor).unwrap_or(cursor);
        <Token![&]>::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            attrs: <_ as Parse>::parse(parser)?,
            and: <_ as Parse>::parse(parser)?,
            mutability: <_ as Parse>::parse(parser)?,
            pat: <_ as Parse>::parse(parser)?,
        })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = Attributes::skip(cursor)?;
        cursor = <Token![&]>::skip(cursor)?;
        cursor = Option::<Token![mut]>::skip(cursor)?;
        Pattern::skip(cursor)
    }
}

impl ToTokens for PatReference {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.and.to_tokens(t);
        self.mutability.to_tokens(t);
        self.pat.to_tokens(t);
    }
}
