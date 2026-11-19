use moxy_token::{Span, Spanner, ToTokenStream, ToTokens, TokenStream};

use crate::*;

/// An or-pattern, e.g. `A | B | C`, with an optional leading `|`.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatOr {
    pub attrs: Attributes,
    pub leading_vert: Option<Token![|]>,
    pub cases: List<Pattern, Token![|]>,
}

impl Attributed for PatOr {
    fn attrs(&self) -> &[Attribute] {
        &self.attrs
    }
}

impl Spanner for PatOr {
    fn span(&self) -> Span {
        self.to_token_stream().span()
    }
}

impl Parse for PatOr {
    fn peek(cursor: Cursor<'_>) -> bool {
        let mut cursor = Attributes::skip(cursor).unwrap_or(cursor);

        if <Token![|]>::peek(cursor) {
            return true;
        }

        while !cursor.is_empty() {
            if <Token![|]>::peek(cursor) {
                return true;
            }

            cursor = cursor.offset(1);
        }

        false
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match <_ as Parse>::parse(parser)? {
            Pattern::Or(value) => Ok(value),
            _ => parser.error("expected or-pattern").into(),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        if Self::peek(cursor) { Pattern::skip(cursor) } else { None }
    }
}

impl ToTokens for PatOr {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.leading_vert.to_tokens(t);
        self.cases.to_tokens(t);
    }
}
