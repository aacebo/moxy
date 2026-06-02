use crate::ast::{Attribute, Defaultness, Signature, StmtBlock, Visibility};
use crate::parse::{ParseError, ParseStream};
use crate::token::ToTokens;
use crate::{Parse, Span, TokenStream};

#[doc = "A free function item (`fn name(...) -> T { ... }`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemFn {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub defaultness: Defaultness,
    pub sig: Signature,
    pub body: StmtBlock,
}

impl Parse for ItemFn {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse_vec::<Attribute>()?;
        let vis = stream.parse::<Visibility>()?;
        let defaultness = Defaultness::Final;
        let sig = stream.parse::<Signature>()?;
        let body = stream.parse::<StmtBlock>()?;
        Ok(ItemFn {
            span: Span::default(),
            attrs,
            vis,
            defaultness,
            sig,
            body,
        })
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
