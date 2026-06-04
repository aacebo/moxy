use moxy_token::parse::{ParseError, ParseStream};
use moxy_token::{Parse, Span, ToTokens, TokenStream};

use crate::{Abi, Attribute, Delimited, ForeignItem, Unsafety};

#[doc = "An `extern` block (`extern \"C\" { ... }`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemForeignMod {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub unsafety: Unsafety,
    pub abi: Abi,
    pub brace: Delimited<Vec<ForeignItem>>,
}

impl Parse for ItemForeignMod {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse::<Vec<Attribute>>()?;
        let unsafety = stream.parse::<Unsafety>()?;
        let abi = stream.parse::<Abi>()?;
        let brace = Delimited::<Vec<ForeignItem>>::parse_brace(stream)?;
        Ok(ItemForeignMod {
            span: Span::default(),
            attrs,
            unsafety,
            abi,
            brace,
        })
    }
}

impl ToTokens for ItemForeignMod {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.unsafety.to_tokens(t);
        self.abi.to_tokens(t);
        self.brace.to_tokens(t);
    }
}
