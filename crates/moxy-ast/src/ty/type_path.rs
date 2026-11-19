use crate::{Cursor, Parse, ParseError, Parser, Type};
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use super::QSelf;
use crate::{Path, PathSegment};

/// A path type (e.g. `T`, `std::vec::Vec`, `<T as Trait>::Item`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TypePath {
    pub qself: Option<QSelf>,
    pub path: Path,
}

impl TypePath {
    /// Skips a path in expression or pattern position, where segments take
    /// arguments only after `::`, so `<T>::f()` ends before the call.
    pub(crate) fn skip_expr(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::skip_with(cursor, Path::skip_expr)
    }

    fn skip_with<'a>(cursor: Cursor<'a>, skip_path: fn(Cursor<'a>) -> Option<Cursor<'a>>) -> Option<Cursor<'a>> {
        if !<Token![<]>::peek(cursor) {
            return skip_path(cursor);
        }

        // The parser's extra level for a qualifier, held to its `>`
        let weight = moxy_token::__private::Depth::enter()?;
        let mut cursor = <Token![<]>::skip(cursor)?;
        cursor = Type::skip(cursor)?;

        if <Token![as]>::peek(cursor) {
            cursor = <Token![as]>::skip(cursor)?;
            cursor = Path::skip(cursor)?;
        }

        cursor = <Token![>]>::skip(cursor)?;
        drop(weight);

        if !<Token![::]>::peek(cursor) {
            return None;
        }

        skip_path(cursor)
    }
}

impl Parse for TypePath {
    fn peek(cursor: Cursor<'_>) -> bool {
        <Token![<]>::peek(cursor) || Path::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        if <Token![<]>::peek(parser.cursor()) {
            let (qself, path) = super::QSelf::parse_qualified(parser)?;

            return Ok(Self {
                qself: Some(qself),
                path,
            });
        }

        Ok(Self {
            qself: None,
            path: <_ as Parse>::parse(parser)?,
        })
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::skip_with(cursor, Path::skip)
    }
}

impl Spanner for TypePath {
    fn span(&self) -> Span {
        if let Some(q) = &self.qself {
            q.span().join(self.path.span())
        } else {
            self.path.span()
        }
    }
}

impl TypePath {
    pub fn emit_segments(segs: &[&PathSegment], tokens: &mut TokenStream) {
        for (i, seg) in segs.iter().enumerate() {
            if i > 0 {
                <Token![::]>::default().to_tokens(tokens);
            }

            seg.to_tokens(tokens);
        }
    }
}

impl ToTokens for TypePath {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        QSelf::path_to_tokens(self.qself.as_ref(), &self.path, tokens);
    }
}
