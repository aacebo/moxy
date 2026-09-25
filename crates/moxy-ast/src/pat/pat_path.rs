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

        !<Token![!]>::peek(cursor)
            && !cursor.is_delimited(moxy_token::Delim::Paren)
            && !cursor.is_delimited(moxy_token::Delim::Brace)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = parser.parse()?;
        let (qself, path) = if <Token![<]>::peek(parser.cursor()) {
            let (qself, path) = QSelf::parse_qualified(parser)?;
            (Some(qself), path)
        } else {
            (None, parser.parse()?)
        };

        Ok(Self { attrs, qself, path })
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        let cursor = Attributes::skip(cursor)?;

        if <Token![<]>::peek(cursor) {
            TypePath::skip(cursor)
        } else {
            Path::skip(cursor)
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
