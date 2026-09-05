use crate::{Parse, ParseError, Parser};
use moxy_token::{LexError, Span, Spanner, ToTokens, TokenStream};

use crate::{Attributes, Defaultness, Signature, StmtBlock, Visibility};

/// A method or associated function inside an `impl` block.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ImplItemFn {
    pub attrs: Attributes,
    pub vis: Visibility,
    pub defaultness: Defaultness,
    pub sig: Signature,
    pub body: StmtBlock,
}

impl Parse for ImplItemFn {
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let at = parser.span();
        let attrs = parser.parse::<Attributes>()?;
        let vis = parser.parse::<Visibility>()?;
        let defaultness = parser.parse::<Defaultness>()?;

        if !crate::sig::Signature::is_start(parser) {
            return Err(LexError::new(at).message("expected impl fn").into());
        }

        let sig = parser.parse::<Signature>()?;
        let body = parser.parse::<StmtBlock>()?;

        Ok(Self {
            attrs,
            vis,
            defaultness,
            sig,
            body,
        })
    }
}

impl Spanner for ImplItemFn {
    fn span(&self) -> Span {
        self.attrs.span().join(self.body.span())
    }
}

impl ToTokens for ImplItemFn {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.vis.to_tokens(t);
        self.defaultness.to_tokens(t);
        self.sig.to_tokens(t);
        self.body.to_tokens(t);
    }
}

impl ImplItemFn {
    pub fn into_impl_item(self) -> super::ImplItem {
        super::ImplItem::from(self)
    }
}
