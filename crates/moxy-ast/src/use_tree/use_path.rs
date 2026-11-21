use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A use path segment (`foo::<rest>`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct UsePath {
    pub prefix: Option<Token![::]>,
    pub segment: UseSegment,
    pub path_sep: Token![::],
    pub tree: Box<UseTree>,
}

impl Parse for UsePath {
    fn peek(cursor: Cursor<'_>) -> bool {
        let cursor = Option::<Token![::]>::skip(cursor).unwrap_or(cursor);
        UseSegment::peek(cursor) && <Token![::]>::peek(cursor.offset(1))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            prefix: <_ as Parse>::parse(parser)?,
            segment: <_ as Parse>::parse(parser)?,
            path_sep: <_ as Parse>::parse(parser)?,
            tree: Box::new(<_ as Parse>::parse(parser)?),
        })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = Option::<Token![::]>::skip(cursor)?;
        cursor = UseSegment::skip(cursor)?;
        cursor = <Token![::]>::skip(cursor)?;
        UseTree::skip(cursor)
    }
}

impl Spanner for UsePath {
    fn span(&self) -> Span {
        self.segment.span().join(self.tree.span())
    }
}

impl ToTokens for UsePath {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.prefix.to_tokens(t);
        self.segment.to_tokens(t);
        self.path_sep.to_tokens(t);
        self.tree.to_tokens(t);
    }
}
