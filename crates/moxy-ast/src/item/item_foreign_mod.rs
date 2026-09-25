use crate::{Parse, ParseError, Parser};
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::{Abi, Attributes, Delimited, ForeignItem, Token};

/// An `extern` block (`extern "C" { ... }`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemForeignMod {
    pub attrs: Attributes,
    pub unsafety: Option<Token![unsafe]>,
    pub abi: Abi,
    pub items: Delimited<Vec<ForeignItem>>,
}

impl Parse for ItemForeignMod {
    fn peek(cursor: crate::Cursor<'_>) -> bool {
        let cursor = Attributes::skip(cursor).unwrap_or(cursor);
        let cursor = Option::<Token![unsafe]>::skip(cursor).unwrap_or(cursor);
        let Some(cursor) = Abi::skip(cursor) else {
            return false;
        };

        cursor.is_delimited(moxy_token::Delim::Brace)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = parser.parse()?;
        let unsafety = parser.parse()?;
        let abi = parser.parse()?;
        let items = Delimited::<Vec<ForeignItem>>::parse_brace(parser)?;

        Ok(Self {
            attrs,
            unsafety,
            abi,
            items,
        })
    }

    fn skip(mut cursor: crate::Cursor<'_>) -> Option<crate::Cursor<'_>> {
        cursor = Attributes::skip(cursor)?;
        cursor = Option::<Token![unsafe]>::skip(cursor)?;
        cursor = Abi::skip(cursor)?;
        let mut inner = cursor.descend(moxy_token::Delim::Brace)?;

        while !inner.is_empty() {
            inner = ForeignItem::skip(inner)?;
        }

        Some(cursor.offset(1))
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
