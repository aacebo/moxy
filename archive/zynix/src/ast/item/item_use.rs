use crate::ast::{Attribute, UseTree, Visibility};
use crate::parse::{ParseError, ParseStream};
use crate::token::ToTokens;
use crate::token::keyword::Use;
use crate::token::punct::Semi;
use crate::{Parse, Span, TokenStream};

#[doc = "A `use` item (`use path::to::Name;`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemUse {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub tree: UseTree,
}

impl Parse for ItemUse {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse_vec::<Attribute>()?;
        let vis = stream.parse::<Visibility>()?;
        let _ = stream.parse::<Use>()?;
        let tree = stream.parse::<UseTree>()?;
        let _ = stream.parse::<Semi>();
        Ok(ItemUse {
            span: Span::default(),
            attrs,
            vis,
            tree,
        })
    }
}

impl ToTokens for ItemUse {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.vis.to_tokens(t);
        Use::default().to_tokens(t);
        self.tree.to_tokens(t);
        Semi::default().to_tokens(t);
    }
}
