use moxy_token::{Span, Spanner, ToTokenStream, ToTokens, TokenStream};

use crate::*;

/// An `extern` block (`extern "C" { ... }`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemForeignMod {
    pub attrs: Attributes,
    pub unsafety: Option<Token![unsafe]>,
    pub abi: Abi,
    /// The inner attributes that open the body, as `#![a]`.
    pub inner_attrs: Attributes,
    pub items: Delimited<Vec<ForeignItem>>,
}

impl Attributed for ItemForeignMod {
    fn attrs(&self) -> &[Attribute] {
        &self.attrs
    }
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
        let attrs = <_ as Parse>::parse(parser)?;
        let unsafety = <_ as Parse>::parse(parser)?;
        let abi = <_ as Parse>::parse(parser)?;
        let mut inner_attrs = Attributes::default();
        let items = Delimited::parse_brace_with(parser, |parser| {
            inner_attrs = Attributes::parse_inner(parser)?;
            <Vec<ForeignItem> as Parse>::parse(parser)
        })?;

        Ok(Self {
            attrs,
            unsafety,
            abi,
            inner_attrs,
            items,
        })
    }

    fn skip(mut cursor: crate::Cursor<'_>) -> Option<crate::Cursor<'_>> {
        cursor = Attributes::skip(cursor)?;
        cursor = Option::<Token![unsafe]>::skip(cursor)?;
        cursor = Abi::skip(cursor)?;
        let mut inner = Attributes::skip_inner(cursor.descend(moxy_token::Delim::Brace)?)?;

        while !inner.is_empty() {
            inner = ForeignItem::skip(inner)?;
        }

        Some(cursor.offset(1))
    }
}

impl Spanner for ItemForeignMod {
    fn span(&self) -> Span {
        self.to_token_stream().span()
    }
}

impl ToTokens for ItemForeignMod {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.unsafety.to_tokens(t);
        self.abi.to_tokens(t);
        let mut body = TokenStream::new();
        self.inner_attrs.to_tokens(&mut body);
        self.items.inner.to_tokens(&mut body);
        self.items.surround(t, body);
    }
}

impl ItemForeignMod {
    pub fn into_item(self) -> super::Item {
        super::Item::from(self)
    }
}
