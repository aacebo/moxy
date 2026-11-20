use moxy_token::{Ident, Span, Spanner, ToTokenStream, ToTokens, TokenStream};

use super::UseTree;
use crate::*;

/// The segments of a use path before its last tree: `a::b::` in
/// `use a::b::{c, d};`.
///
/// The segments are held in order in one node, so every traversal of a long
/// path runs without recursing once per segment.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct UsePathChain {
    pub prefix: Option<Token![::]>,
    /// One or more segments, each followed by `::`.
    pub segments: List<Ident, Token![::]>,
    pub tree: Box<UseTree>,
}

impl Parse for UsePathChain {
    fn peek(cursor: Cursor<'_>) -> bool {
        let cursor = Option::<Token![::]>::skip(cursor).unwrap_or(cursor);
        super::peek_segment(cursor) && <Token![::]>::peek(cursor.offset(1))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let prefix = <_ as Parse>::parse(parser)?;
        let mut segments = List::new();

        loop {
            segments.push_value(parser.parse_ident_any()?);
            segments.push_punct(<_ as Parse>::parse(parser)?);

            if !(super::peek_segment(parser.cursor()) && <Token![::]>::peek(parser.cursor().offset(1))) {
                break;
            }
        }

        Ok(Self {
            prefix,
            segments,
            tree: Box::new(<_ as Parse>::parse(parser)?),
        })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = Option::<Token![::]>::skip(cursor)?;

        loop {
            cursor = super::skip_segment(cursor)?;
            cursor = <Token![::]>::skip(cursor)?;

            if !(super::peek_segment(cursor) && <Token![::]>::peek(cursor.offset(1))) {
                break;
            }
        }

        UseTree::skip(cursor)
    }
}

impl Spanner for UsePathChain {
    fn span(&self) -> Span {
        self.to_token_stream().span()
    }
}

impl ToTokens for UsePathChain {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.prefix.to_tokens(t);
        self.segments.to_tokens(t);
        self.tree.to_tokens(t);
    }
}
