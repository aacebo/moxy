use moxy_token::Token;
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::{Parse, Span, Spanner, ToTokens, TokenStream};

use crate::{Attributes, UseTree, Visibility};

/// A `use` item (`use path::to::Name;`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemUse {
    pub attrs: Attributes,
    pub vis: Visibility,
    pub use_keyword: Token![use],
    pub tree: UseTree,
    pub semi_punct: Token![;],
}

impl Parse for ItemUse {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse::<Attributes>()?;
        let vis = stream.parse::<Visibility>()?;
        let use_keyword = stream.parse::<Token![use]>()?;
        let tree = stream.parse::<UseTree>()?;
        let semi_punct = stream.parse::<Token![;]>().unwrap_or_default();

        Ok(Self {
            attrs,
            vis,
            use_keyword,
            tree,
            semi_punct,
        })
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
