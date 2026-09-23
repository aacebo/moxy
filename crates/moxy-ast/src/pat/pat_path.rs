use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::ty::TypePath;
use crate::*;

/// A path pattern, e.g. `Some` or `std::option::Option::None`.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatPath {
    pub attrs: Attributes,
    pub qself: Option<QSelf>,
    pub path: Path,
}

impl Spanner for PatPath {
    fn span(&self) -> Span {
        self.attrs.span().join(self.path.span())
    }
}

impl Parse for PatPath {
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

        !cursor.peek::<Token![!]>()
            && !cursor.is_delimited(moxy_token::Delim::Paren)
            && !cursor.is_delimited(moxy_token::Delim::Brace)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = parser.parse()?;
        let (qself, path) = if parser.peek::<Token![<]>() {
            let (qself, path) = QSelf::parse_qualified(parser)?;
            (Some(qself), path)
        } else {
            (None, parser.parse()?)
        };

        Ok(Self { attrs, qself, path })
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        let cursor = Attributes::skip(cursor)?;

        if cursor.peek::<Token![<]>() {
            cursor.skip::<TypePath>()
        } else {
            cursor.skip::<Path>()
        }
    }
}

impl ToTokens for PatPath {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.qself.to_tokens(t);
        self.path.to_tokens(t);
    }
}
