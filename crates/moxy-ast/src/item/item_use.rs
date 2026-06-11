use moxy_token::keyword::Use;
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::Semi;
use moxy_token::{Parse, Span, Spanner, ToTokens, TokenStream};

use crate::{Attributes, UseTree, Visibility};

/// A `use` item (`use path::to::Name;`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemUse {
    pub attrs: Attributes,
    pub vis: Visibility,
    pub use_keyword: Use,
    pub tree: UseTree,
    pub semi_punct: Semi,
}

impl Parse for ItemUse {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse::<Attributes>()?;
        let vis = stream.parse::<Visibility>()?;
        let use_keyword = stream.parse::<Use>()?;
        let tree = stream.parse::<UseTree>()?;
        let semi_punct = stream.parse::<Semi>().unwrap_or_default();
        Ok(ItemUse {
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
        let start = if let Some(a) = self.attrs.first() {
            a.span()
        } else if !matches!(self.vis, Visibility::Inherited) {
            self.vis.span()
        } else {
            self.use_keyword.span()
        };
        start.join(self.semi_punct.span())
    }
}

impl ToTokens for ItemUse {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
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
