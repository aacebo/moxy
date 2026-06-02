use moxy_token::keyword::Use;
use moxy_token::parse::{ParseError, ParseStream};
use moxy_token::punct::Semi;
use moxy_token::{Parse, Span, ToTokens, TokenStream};

use crate::{Attribute, UseTree, Visibility};

#[doc = "A `use` item (`use path::to::Name;`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemUse {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub use_keyword: Use,
    pub tree: UseTree,
    pub semi_punct: Semi,
}

impl Parse for ItemUse {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse_vec::<Attribute>()?;
        let vis = stream.parse::<Visibility>()?;
        let use_keyword = stream.parse::<Use>()?;
        let tree = stream.parse::<UseTree>()?;
        let semi_punct = stream.parse::<Semi>().unwrap_or_default();
        Ok(ItemUse {
            span: Span::default(),
            attrs,
            vis,
            use_keyword,
            tree,
            semi_punct,
        })
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
