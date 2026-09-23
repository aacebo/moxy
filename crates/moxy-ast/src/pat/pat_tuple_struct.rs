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
        let cursor = if cursor.peek::<Token![<]>() {
            let Some(cursor) = cursor.skip::<TypePath>() else {
                return false;
            };

            cursor
        } else {
            let Some(cursor) = cursor.skip::<Path>() else {
                return false;
            };

            cursor
        };

        cursor.is_delimited(Delim::Paren)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = parser.parse()?;
        let (qself, path) = if parser.peek::<Token![<]>() {
            let (qself, path) = QSelf::parse_qualified(parser)?;
            (Some(qself), path)
        } else {
            (None, parser.parse()?)
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
        cursor = if cursor.peek::<Token![<]>() {
            cursor.skip::<TypePath>()?
        } else {
            cursor.skip::<Path>()?
        };

        let mut inner = cursor.descend(Delim::Paren)?;

        while !inner.is_empty() {
            inner = inner.skip::<Pattern>()?;

            if inner.is_empty() {
                break;
            }

            inner = inner.skip::<Token![,]>()?;
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
