use moxy_token::{Ident, Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// An enum item (`enum Name<T> { Variant, ... }`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemEnum {
    pub attrs: Attributes,
    pub vis: Visibility,
    pub enum_keyword: Token![enum],
    pub ident: Ident,
    pub generics: Generics,
    pub variants: Delimited<List<Variant, Token![,]>>,
}

impl Attributed for ItemEnum {
    fn attrs(&self) -> &[Attribute] {
        &self.attrs
    }
}

impl Parse for ItemEnum {
    fn peek(cursor: crate::Cursor<'_>) -> bool {
        let cursor = Attributes::skip(cursor).unwrap_or(cursor);
        let cursor = Visibility::skip(cursor).unwrap_or(cursor);
        <Token![enum]>::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = <_ as Parse>::parse(parser)?;
        let vis = <_ as Parse>::parse(parser)?;
        let enum_keyword = <_ as Parse>::parse(parser)?;
        let ident = <_ as Parse>::parse(parser)?;
        let generics = <_ as Parse>::parse(parser)?;
        let variants = Delimited::parse_brace_with(parser, List::parse_terminated)?;

        Ok(Self {
            attrs,
            vis,
            enum_keyword,
            ident,
            generics,
            variants,
        })
    }

    fn skip(mut cursor: crate::Cursor<'_>) -> Option<crate::Cursor<'_>> {
        cursor = Attributes::skip(cursor)?;
        cursor = Visibility::skip(cursor)?;
        cursor = <Token![enum]>::skip(cursor)?;
        cursor = Ident::skip(cursor)?;
        cursor = Generics::skip(cursor)?;
        let mut inner = cursor.descend(moxy_token::Delim::Brace)?;

        while !inner.is_empty() {
            inner = Variant::skip(inner)?;

            if inner.is_empty() {
                break;
            }

            inner = <Token![,]>::skip(inner)?;
        }

        Some(cursor.offset(1))
    }
}

impl Spanner for ItemEnum {
    fn span(&self) -> Span {
        self.attrs.span().join(self.variants.span())
    }
}

impl ToTokens for ItemEnum {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.vis.to_tokens(t);
        self.enum_keyword.to_tokens(t);
        self.ident.to_tokens(t);
        self.generics.to_tokens(t);
        self.variants.to_tokens(t);
    }
}

impl ItemEnum {
    pub fn into_item(self) -> super::Item {
        super::Item::from(self)
    }
}
