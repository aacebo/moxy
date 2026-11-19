use moxy_token::{Ident, Span, Spanner, ToTokenStream, ToTokens, TokenStream};

use super::UseTree;
use crate::*;

/// A use path segment (`foo::<rest>`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct UsePath {
    pub prefix: Option<Token![::]>,
    pub ident: Ident,
    pub path_sep: Token![::],
    pub tree: Box<UseTree>,
}

impl Parse for UsePath {
    fn peek(cursor: Cursor<'_>) -> bool {
        let cursor = Option::<Token![::]>::skip(cursor).unwrap_or(cursor);
        super::peek_segment(cursor) && <Token![::]>::peek(cursor.offset(1))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let prefix = <_ as Parse>::parse(parser)?;
        let ident = parser.parse_ident_any()?;
        let path_sep = <_ as Parse>::parse(parser)?;

        // `crate` names the crate root, so only the first segment can be it
        if parser.curr().is_some_and(|token| token.is_keyword_crate()) {
            return parser.error("`crate` can only begin a path").into();
        }

        Ok(Self {
            prefix,
            ident,
            path_sep,
            tree: Box::new(<_ as Parse>::parse(parser)?),
        })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = Option::<Token![::]>::skip(cursor)?;
        cursor = super::skip_segment(cursor)?;
        cursor = <Token![::]>::skip(cursor)?;

        if cursor.curr().is_some_and(|token| token.is_keyword_crate()) {
            return None;
        }

        UseTree::skip(cursor)
    }
}

impl Spanner for UsePath {
    fn span(&self) -> Span {
        self.to_token_stream().span()
    }
}

impl ToTokens for UsePath {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.prefix.to_tokens(t);
        self.ident.to_tokens(t);
        self.path_sep.to_tokens(t);
        self.tree.to_tokens(t);
    }
}
