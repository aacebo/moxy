use moxy_token::{Delim, Span, Spanner, ToTokens, TokenStream};

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
    pub elems: Delimited<Punctuated<Pattern, Token![,]>>,
}

impl Spanner for PatTupleStruct {
    fn span(&self) -> Span {
        self.attrs.span().join(self.elems.span())
    }
}

impl Parse for PatTupleStruct {
    fn peek(cursor: Cursor<'_>) -> bool {
        let cursor = Attributes::skip(cursor).unwrap_or(cursor);
        let cursor = if <Token![<]>::peek(cursor) {
            let Some(cursor) = TypePath::skip(cursor) else {
                return false;
            };

            cursor
        } else {
            let Some(cursor) = Path::skip(cursor) else {
                return false;
            };

            cursor
        };

        cursor.is_delimited(Delim::Paren)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = <_ as Parse>::parse(parser)?;
        let (qself, path) = if <Token![<]>::peek(parser.cursor()) {
            let (qself, path) = QSelf::parse_qualified(parser)?;
            (Some(qself), path)
        } else {
            (None, <_ as Parse>::parse(parser)?)
        };

        let (span, parser) = parser.parse_group_spanned(Delim::Paren)?;

        Ok(Self {
            attrs,
            qself,
            path,
            elems: Delimited::paren(span, Punctuated::parse_terminated(&parser)?),
        })
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        let mut cursor = Attributes::skip(cursor)?;
        cursor = if <Token![<]>::peek(cursor) {
            TypePath::skip(cursor)?
        } else {
            Path::skip(cursor)?
        };

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

impl ToTokens for PatTupleStruct {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.qself.to_tokens(t);
        self.path.to_tokens(t);
        self.elems.to_tokens(t);
    }
}
