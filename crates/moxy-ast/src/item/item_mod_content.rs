use moxy_token::{Delim, Span, Spanner, ToTokenStream, ToTokens, TokenStream};

use crate::*;

/// The braced body of a module, `{ #![a] fn f() {} }` in `mod m { .. }`, with
/// the inner attributes that open it.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemModContent {
    /// The inner attributes that open the body, as `#![a]`.
    pub inner_attrs: Attributes,
    pub items: Delimited<Vec<Item>>,
}

impl Parse for ItemModContent {
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.is_delimited(Delim::Brace)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let mut inner_attrs = Attributes::default();
        let items = Delimited::parse_brace_with(parser, |parser| {
            inner_attrs = Attributes::parse_inner(parser)?;
            <Vec<Item> as Parse>::parse(parser)
        })?;

        Ok(Self { inner_attrs, items })
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        let mut inner = Attributes::skip_inner(cursor.descend(Delim::Brace)?)?;

        while !inner.is_empty() {
            inner = Item::skip(inner)?;
        }

        Some(cursor.offset(1))
    }
}

impl Spanner for ItemModContent {
    fn span(&self) -> Span {
        self.to_token_stream().span()
    }
}

impl ToTokens for ItemModContent {
    fn to_tokens(&self, t: &mut TokenStream) {
        let mut body = TokenStream::new();
        self.inner_attrs.to_tokens(&mut body);
        self.items.inner.to_tokens(&mut body);
        self.items.surround(t, body);
    }
}
