use moxy_token::{Delim, Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A braced use group (`{a, b::c}`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct UseGroup {
    pub items: Delimited<Punctuated<UseTree, Token![,]>>,
}

impl Parse for UseGroup {
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.descend(Delim::Brace).map(|c| c.peek::<UseTree>()).unwrap_or_default()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let (span, parser) = parser.parse_group_spanned(Delim::Brace)?;

        Ok(Self {
            items: Delimited::brace(span, Punctuated::parse_separated_nonempty(&parser)?),
        })
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        let mut inner = cursor.descend(Delim::Brace)?;
        inner = inner.skip::<UseTree>()?;

        while inner.peek::<Token![,]>() {
            inner = inner.skip::<Token![,]>()?;
            inner = inner.skip::<UseTree>()?;
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
