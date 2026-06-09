use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::{Parse, Span, Spanner, ToTokens, TokenStream};

use crate::{Attribute, Defaultness, Signature, StmtBlock, Visibility};

/// A free function item (`fn name(...) -> T { ... }`).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemFn {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub defaultness: Defaultness,
    pub sig: Signature,
    pub body: StmtBlock,
}

impl Parse for ItemFn {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse::<Vec<Attribute>>()?;
        let vis = stream.parse::<Visibility>()?;
        let defaultness = Defaultness::Final;
        let sig = stream.parse::<Signature>()?;
        let body = stream.parse::<StmtBlock>()?;
        Ok(ItemFn {
            attrs,
            vis,
            defaultness,
            sig,
            body,
        })
    }
}

impl Spanner for ItemFn {
    fn span(&self) -> Span {
        let start = if let Some(a) = self.attrs.first() {
            a.span()
        } else if !matches!(self.vis, Visibility::Inherited) {
            self.vis.span()
        } else {
            self.sig.span()
        };
        start.join(self.body.span())
    }
}

impl ToTokens for ItemFn {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.vis.to_tokens(t);
        self.defaultness.to_tokens(t);
        self.sig.to_tokens(t);
        self.body.to_tokens(t);
    }
}

impl ItemFn {
    pub fn into_item(self) -> super::Item {
        super::Item::from(self)
    }
}
