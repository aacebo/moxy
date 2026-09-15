use moxy_token::{Delim, Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A parenthesized pattern, e.g. `(A | B)`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatParen {
    pub attrs: Attributes,
    pub content: Delimited<Box<Pattern>>,
}

impl Spanner for PatParen {
    fn span(&self) -> Span {
        self.content.span()
    }
}

impl Parse for PatParen {
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.is_delimited(Delim::Paren)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            attrs: parser.parse()?,
            content: parser.parse()?,
        })
    }
}

impl ToTokens for PatParen {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.content.to_tokens(t);
    }
}
