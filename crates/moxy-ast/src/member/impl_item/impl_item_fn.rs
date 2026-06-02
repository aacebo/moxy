use moxy_token::parse::{ParseError, ParseStream};
use moxy_token::{LexError, Parse, Span, ToTokens, TokenStream};

use super::ImplItem;
use crate::{Attribute, Defaultness, Signature, StmtBlock, Visibility};

#[doc = "A method or associated function inside an `impl` block."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ImplItemFn {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub defaultness: Defaultness,
    pub sig: Signature,
    pub body: StmtBlock,
}

impl Parse for ImplItemFn {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let at = stream.span();
        let attrs = stream.parse::<Vec<Attribute>>()?;
        let vis = stream.parse::<Visibility>()?;
        let defaultness = stream.parse::<Defaultness>()?;

        if !crate::sig::Signature::is_start(stream) {
            return Err(LexError::new(at).message("expected impl fn").into());
        }

        let sig = stream.parse::<Signature>()?;
        let body = stream.parse::<StmtBlock>()?;
        Ok(ImplItemFn {
            span: Span::default(),
            attrs,
            vis,
            defaultness,
            sig,
            body,
        })
    }
}

impl ToTokens for ImplItemFn {
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
