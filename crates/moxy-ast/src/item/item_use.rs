use crate::Token;
use crate::{Parse, ParseError, Parser};
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::{Attributes, UseTree, Visibility};

/// A `use` item (`use path::to::Name;`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemUse {
    pub attrs: Attributes,
    pub vis: Visibility,
    pub use_keyword: Token![use],
    pub tree: UseTree,
    pub semi_punct: Token![;],
}

impl Parse for ItemUse {
    fn peek(cursor: crate::Cursor<'_>) -> bool {
        let cursor = Attributes::skip(cursor).unwrap_or(cursor);
        let cursor = Visibility::skip(cursor).unwrap_or(cursor);
        cursor.peek::<Token![use]>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = parser.parse()?;
        let vis = parser.parse()?;
        let use_keyword = parser.parse()?;
        let tree = parser.parse()?;
        let semi_punct = parser.parse()?;

        Ok(Self {
            attrs,
            vis,
            use_keyword,
            tree,
            semi_punct,
        })
    }

    fn skip(mut cursor: crate::Cursor<'_>) -> Option<crate::Cursor<'_>> {
        cursor = Attributes::skip(cursor)?;
        cursor = Visibility::skip(cursor)?;
        cursor = cursor.skip::<Token![use]>()?;
        cursor = cursor.skip::<UseTree>()?;
        cursor.skip::<Token![;]>()
    }
}

impl Spanner for ItemUse {
    fn span(&self) -> Span {
        self.attrs.span().join(self.semi_punct.span())
    }
}

impl ToTokens for ItemUse {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.vis.to_tokens(t);
        self.use_keyword.to_tokens(t);
        self.tree.to_tokens(t);
        self.semi_punct.to_tokens(t);
    }
}

impl ItemUse {
    pub fn into_item(self) -> super::Item {
        super::Item::from(self)
    }
}
