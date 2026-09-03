use crate::{Parse, ParseError, Parser};
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::{Abi, Attributes, Delimited, ForeignItem, Unsafety};

/// An `extern` block (`extern "C" { ... }`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemForeignMod {
    pub attrs: Attributes,
    pub unsafety: Unsafety,
    pub abi: Abi,
    pub items: Delimited<Vec<ForeignItem>>,
}

impl Parse for ItemForeignMod {
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = parser.parse::<Attributes>()?;
        let unsafety = parser.parse::<Unsafety>()?;
        let abi = parser.parse::<Abi>()?;
        let items = Delimited::<Vec<ForeignItem>>::parse_brace(parser)?;

        Ok(Self {
            attrs,
            unsafety,
            abi,
            items,
        })
    }
}

impl Spanner for ItemForeignMod {
    fn span(&self) -> Span {
        self.attrs.span().join(self.items.span())
    }
}

impl ToTokens for ItemForeignMod {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.unsafety.to_tokens(t);
        self.abi.to_tokens(t);
        self.items.to_tokens(t);
    }
}

impl ItemForeignMod {
    pub fn into_item(self) -> super::Item {
        super::Item::from(self)
    }
}
