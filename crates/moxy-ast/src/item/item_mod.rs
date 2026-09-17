use crate::Token;
use crate::{Parse, ParseError, Parser};
use moxy_token::{Delim, Span, Spanner, ToTokens, TokenStream, TokenTree};

use super::Item;
use crate::{Attributes, Delimited, Ident, Unsafety, Visibility};

/// A module item (`mod foo;` or `mod foo { ... }`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemMod {
    pub attrs: Attributes,
    pub vis: Visibility,
    pub unsafety: Unsafety,
    pub mod_keyword: Token![mod],
    pub ident: Ident,
    pub content: Option<Delimited<Vec<Item>>>,
    pub semi_punct: Option<Token![;]>,
}

impl Parse for ItemMod {
    fn peek(cursor: crate::Cursor<'_>) -> bool {
        let cursor = Attributes::skip(cursor).unwrap_or(cursor);
        let cursor = Visibility::skip(cursor).unwrap_or(cursor);
        let cursor = Unsafety::skip(cursor).unwrap_or(cursor);
        cursor.peek::<Token![mod]>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = parser.parse()?;
        let vis = parser.parse()?;
        let unsafety = parser.parse()?;
        let mod_keyword = parser.parse()?;
        let ident = parser.parse()?;
        let (content, semi_punct) = if matches!(parser.curr(), Some(TokenTree::Group(g)) if g.delim() == Delim::Brace) {
            let brace = Delimited::<Vec<Item>>::parse_brace(parser)?;
            (Some(brace), None)
        } else {
            let semi_punct = parser.parse()?;
            (None, Some(semi_punct))
        };

        Ok(Self {
            attrs,
            vis,
            unsafety,
            mod_keyword,
            ident,
            content,
            semi_punct,
        })
    }

    fn skip(mut cursor: crate::Cursor<'_>) -> Option<crate::Cursor<'_>> {
        cursor = Attributes::skip(cursor)?;
        cursor = Visibility::skip(cursor)?;
        cursor = Unsafety::skip(cursor)?;
        cursor = cursor.skip::<Token![mod]>()?;
        cursor = cursor.skip::<Ident>()?;

        if cursor.is_delimited(Delim::Brace) {
            let mut inner = cursor.descend(Delim::Brace)?;

            while !inner.is_empty() {
                inner = inner.skip::<Item>()?;
            }

            Some(cursor.offset(1))
        } else {
            cursor.skip::<Token![;]>()
        }
    }
}

impl Spanner for ItemMod {
    fn span(&self) -> Span {
        let end = if let Some(c) = &self.content {
            c.span()
        } else if let Some(s) = &self.semi_punct {
            s.span()
        } else {
            self.mod_keyword.span()
        };

        self.attrs.span().join(end)
    }
}

impl ToTokens for ItemMod {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.vis.to_tokens(t);
        self.mod_keyword.to_tokens(t);
        self.ident.to_tokens(t);

        match &self.content {
            Some(brace) => brace.to_tokens(t),
            None => self.semi_punct.to_tokens(t),
        }
    }
}

impl ItemMod {
    pub fn into_item(self) -> super::Item {
        super::Item::from(self)
    }
}
