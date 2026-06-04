use moxy_token::parse::{ParseError, ParseStream};
use moxy_token::{Brace, Parse, Span, ToTokens, TokenStream};

use crate::{Abi, Attribute, ForeignItem, Unsafety};

#[doc = "An `extern` block (`extern \"C\" { ... }`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemForeignMod {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub unsafety: Unsafety,
    pub abi: Abi,
    pub brace: Brace,
    pub items: Vec<ForeignItem>,
}

impl Parse for ItemForeignMod {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse::<Vec<Attribute>>()?;
        let unsafety = stream.parse::<Unsafety>()?;
        let abi = stream.parse::<Abi>()?;
        let (brace, group) = stream.parse_brace()?;
        let mut inner = group.parse();
        let items = inner.parse::<Vec<ForeignItem>>()?;
        Ok(ItemForeignMod {
            span: Span::default(),
            attrs,
            unsafety,
            abi,
            brace,
            items,
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
        let mut inner = TokenStream::new();

        for it in &self.items {
            it.to_tokens(&mut inner);
        }

        self.brace.surround(t, inner);
    }
}
