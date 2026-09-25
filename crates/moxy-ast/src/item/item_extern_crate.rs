use crate::Token;
use crate::{Parse, ParseError, Parser};
use moxy_token::{Ident, Span, Spanner, ToTokens, TokenStream};

use crate::{Attributes, Visibility};

/// An `extern crate` item (`extern crate foo;` or `extern crate foo as bar;`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemExternCrate {
    pub attrs: Attributes,
    pub vis: Visibility,
    pub extern_keyword: Token![extern],
    pub crate_keyword: Token![crate],
    pub ident: Ident,
    pub as_keyword: Option<Token![as]>,
    pub rename: Option<Ident>,
    pub semi_punct: Token![;],
}

impl Parse for ItemExternCrate {
    fn peek(cursor: crate::Cursor<'_>) -> bool {
        let cursor = Attributes::skip(cursor).unwrap_or(cursor);
        let cursor = Visibility::skip(cursor).unwrap_or(cursor);
        <Token![extern]>::peek(cursor) && <Token![crate]>::peek(cursor.offset(1))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = <_ as Parse>::parse(parser)?;
        let vis = <_ as Parse>::parse(parser)?;
        let extern_keyword = <_ as Parse>::parse(parser)?;
        let crate_keyword = <_ as Parse>::parse(parser)?;
        let ident = <_ as Parse>::parse(parser)?;
        let (as_keyword, rename) = if <Token![as]>::peek(parser.cursor()) {
            let as_keyword = <_ as Parse>::parse(parser)?;
            let rename = <_ as Parse>::parse(parser)?;
            (Some(as_keyword), Some(rename))
        } else {
            (None, None)
        };

        let semi_punct = <_ as Parse>::parse(parser)?;

        Ok(Self {
            attrs,
            vis,
            extern_keyword,
            crate_keyword,
            ident,
            as_keyword,
            rename,
            semi_punct,
        })
    }

    fn skip(mut cursor: crate::Cursor<'_>) -> Option<crate::Cursor<'_>> {
        cursor = Attributes::skip(cursor)?;
        cursor = Visibility::skip(cursor)?;
        cursor = <Token![extern]>::skip(cursor)?;
        cursor = <Token![crate]>::skip(cursor)?;
        cursor = Ident::skip(cursor)?;

        if <Token![as]>::peek(cursor) {
            cursor = <Token![as]>::skip(cursor)?;
            cursor = Ident::skip(cursor)?;
        }

        <Token![;]>::skip(cursor)
    }
}

impl Spanner for ItemExternCrate {
    fn span(&self) -> Span {
        self.attrs.span().join(self.semi_punct.span())
    }
}

impl ToTokens for ItemExternCrate {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.vis.to_tokens(t);
        self.extern_keyword.to_tokens(t);
        self.crate_keyword.to_tokens(t);
        self.ident.to_tokens(t);

        if let (Some(as_keyword), Some(r)) = (&self.as_keyword, &self.rename) {
            as_keyword.to_tokens(t);
            r.to_tokens(t);
        }

        self.semi_punct.to_tokens(t);
    }
}

impl ItemExternCrate {
    pub fn into_item(self) -> super::Item {
        super::Item::from(self)
    }
}
