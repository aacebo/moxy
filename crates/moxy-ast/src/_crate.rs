use crate::{Parse, ParseError, Parser};
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::{Attributes, Item};

/// A whole parsed crate (inner attributes + items).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Crate {
    pub attrs: Attributes,
    pub items: Vec<Item>,
}

impl Spanner for Crate {
    fn span(&self) -> Span {
        let start = self
            .attrs
            .first()
            .map(|a| a.span())
            .or_else(|| self.items.first().map(|i| i.span()));

        let end = self
            .items
            .last()
            .map(|i| i.span())
            .or_else(|| self.attrs.last().map(|a| a.span()));

        match (start, end) {
            (Some(s), Some(e)) => s.join(e),
            (Some(s), None) => s,
            (None, Some(e)) => e,
            (None, None) => Span::call_site(),
        }
    }
}

impl Parse for Crate {
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = parser.parse::<Attributes>()?;
        let items = parser.parse::<Vec<Item>>()?;
        Ok(Self { attrs, items })
    }
}

impl ToTokens for Crate {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);

        for it in &self.items {
            it.to_tokens(t);
        }
    }
}
