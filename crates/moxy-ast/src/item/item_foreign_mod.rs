use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::{Parse, Span, Spanner, ToTokens, TokenStream};

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
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse::<Attributes>()?;
        let unsafety = stream.parse::<Unsafety>()?;
        let abi = stream.parse::<Abi>()?;
        let items = Delimited::<Vec<ForeignItem>>::parse_brace(stream)?;
        Ok(ItemForeignMod {
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
