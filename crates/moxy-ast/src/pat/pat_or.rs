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
    // An or-pattern starts with `|` or with a pattern that `|` follows; the
    // check reads no further than that first pattern
    fn peek(cursor: Cursor<'_>) -> bool {
        <Token![|]>::peek(cursor) || super::skip::single(cursor).is_some_and(|cursor| <Token![|]>::peek(cursor))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match <_ as Parse>::parse(parser)? {
            Pattern::Or(value) => Ok(value),
            other => Err(ParseError::new(
                other.span(),
                format!("expected or-pattern, found {}", found_node(&other)),
            )),
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
