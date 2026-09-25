use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// An or-pattern, e.g. `A | B | C`.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatOr {
    pub attrs: Attributes,
    pub cases: Punctuated<Pattern, Token![|]>,
}

impl Spanner for PatOr {
    fn span(&self) -> Span {
        let cases = match (self.cases.first(), self.cases.last()) {
            (Some(a), Some(b)) => a.span().join(b.span()),
            _ => Span::call_site(),
        };

        self.attrs.span().join(cases)
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
        match parser.parse()? {
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
        self.cases.to_tokens(t);
    }
}
