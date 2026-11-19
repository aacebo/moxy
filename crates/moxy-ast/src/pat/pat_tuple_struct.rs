use moxy_token::{Delim, Span, Spanner, ToTokenStream, ToTokens, TokenStream};

use crate::ty::TypePath;
use crate::*;

/// A tuple-struct pattern, e.g. `Point(x, y)`.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatTupleStruct {
    pub attrs: Attributes,
    pub qself: Option<QSelf>,
    pub path: Path,
    pub elems: Delimited<List<Pattern, Token![,]>>,
}

impl Attributed for PatTupleStruct {
    fn attrs(&self) -> &[Attribute] {
        &self.attrs
    }
}

impl Spanner for PatTupleStruct {
    fn span(&self) -> Span {
        self.to_token_stream().span()
    }
}

impl PatTupleStruct {
    // Skips the parenthesized elements after the path, with the level their
    // parse takes
    pub(crate) fn skip_elems(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        let _weight = moxy_token::__private::Depth::enter()?;
        let mut inner = cursor.descend(Delim::Paren)?;

        while !inner.is_empty() {
            inner = Pattern::skip(inner)?;

            if inner.is_empty() {
                break;
            }

            inner = <Token![,]>::skip(inner)?;
        }

        Some(cursor.offset(1))
    }
}

impl Parse for PatTupleStruct {
    fn peek(cursor: Cursor<'_>) -> bool {
        let cursor = Attributes::skip(cursor).unwrap_or(cursor);
        let cursor = if <Token![<]>::peek(cursor) {
            let Some(cursor) = TypePath::skip_expr(cursor) else {
                return false;
            };

            cursor
        } else {
            let Some(cursor) = Path::skip_expr(cursor) else {
                return false;
            };

            cursor
        };

        cursor.is_delimited(Delim::Paren)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = <_ as Parse>::parse(parser)?;
        let (qself, path) = if <Token![<]>::peek(parser.cursor()) {
            let (qself, path) = QSelf::parse_qualified_expr(parser)?;
            (Some(qself), path)
        } else {
            (None, Path::parse_expr(parser)?)
        };

        // The elements take a second level: a pattern nested in them takes
        // more stack than the level it nests. `skip_elems` takes the same
        let _weight = moxy_token::__private::Depth::enter().ok_or_else(|| ParseError::too_deep(parser.span()))?;
        let (span, parser) = parser.parse_group_spanned(Delim::Paren)?;

        Ok(Self {
            attrs,
            qself,
            path,
            elems: Delimited::paren(span, List::parse_all(&parser)?),
        })
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        let mut cursor = Attributes::skip(cursor)?;
        cursor = if <Token![<]>::peek(cursor) {
            TypePath::skip_expr(cursor)?
        } else {
            Path::skip_expr(cursor)?
        };

        Self::skip_elems(cursor)
    }
}

impl ToTokens for PatTupleStruct {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        QSelf::path_to_tokens(self.qself.as_ref(), &self.path, t);
        self.elems.to_tokens(t);
    }
}
