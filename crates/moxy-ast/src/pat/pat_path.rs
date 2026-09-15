use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A path pattern, e.g. `Some` or `std::option::Option::None`.
#[derive(Debug, Clone, PartialEq, Eq)]
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
        cursor.peek::<QSelf>() || cursor.peek::<Path>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            attrs: parser.parse()?,
            qself: parser.parse()?,
            path: parser.parse()?,
        })
    }
}

impl ToTokens for PatPath {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.qself.to_tokens(t);
        self.path.to_tokens(t);
    }
}
