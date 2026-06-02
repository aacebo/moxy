use moxy_token::parse::{ParseError, ParseStream};
use moxy_token::{Delim, Parse, Span, ToTokens, TokenStream};

use crate::{Abi, Attribute, ForeignItem, Unsafety};

#[doc = "An `extern` block (`extern \"C\" { ... }`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemForeignMod {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub unsafety: Unsafety,
    pub abi: Abi,
    pub items: Vec<ForeignItem>,
}

impl Parse for ItemForeignMod {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse_vec::<Attribute>()?;
        let unsafety = stream.parse::<Unsafety>()?;
        let abi = stream.parse::<Abi>()?;
        let group = stream.parse_group(Delim::Brace)?;
        let mut inner = group.parse();
        let items = inner.parse_vec::<ForeignItem>()?;
        Ok(ItemForeignMod {
            span: Span::default(),
            attrs,
            unsafety,
            abi,
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

        t.extend_one(moxy_token::TokenTree::Group(moxy_token::Group::new(
            moxy_token::Delim::Brace,
            inner,
        )));
    }
}
