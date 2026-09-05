use crate::{Parse, ParseError, Parser};
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::{Attributes, Signature, StmtBlock, Visibility};

/// A free function item (`fn name(...) -> T { ... }`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemFn {
    pub attrs: Attributes,
    pub vis: Visibility,
    pub sig: Signature,
    pub body: StmtBlock,
}

impl Parse for ItemFn {
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = parser.parse::<Attributes>()?;
        let vis = parser.parse::<Visibility>()?;
        let sig = parser.parse::<Signature>()?;
        let body = parser.parse::<StmtBlock>()?;
        Ok(Self { attrs, vis, sig, body })
    }
}

impl Spanner for ItemFn {
    fn span(&self) -> Span {
        self.attrs.span().join(self.body.span())
    }
}

impl ToTokens for ItemFn {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.vis.to_tokens(t);
        self.sig.to_tokens(t);
        self.body.to_tokens(t);
    }
}

impl ItemFn {
    pub fn into_item(self) -> super::Item {
        super::Item::from(self)
    }
}
