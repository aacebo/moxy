use moxy_token::{Delim, Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A braced use group (`{a, b::c}`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct UseGroup {
    pub items: Delimited<Punctuated<UseTree, Token![,]>>,
}

impl Parse for UseGroup {
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.descend(Delim::Brace).map(|c| UseTree::peek(c)).unwrap_or_default()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let (span, parser) = parser.parse_group_spanned(Delim::Brace)?;

        Ok(Self {
            items: Delimited::brace(span, Punctuated::parse_separated_nonempty(&parser)?),
        })
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        let mut inner = cursor.descend(Delim::Brace)?;
        inner = UseTree::skip(inner)?;

        while <Token![,]>::peek(inner) {
            inner = <Token![,]>::skip(inner)?;
            inner = UseTree::skip(inner)?;
        }

        if inner.is_empty() { Some(cursor.offset(1)) } else { None }
    }
}

impl Spanner for UseGroup {
    fn span(&self) -> Span {
        self.items.span()
    }
}

impl ToTokens for UseGroup {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.items.to_tokens(t);
    }
}
