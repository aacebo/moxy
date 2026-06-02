use crate::ast::{Attribute, Item};
use crate::parse::{ParseError, ParseStream};
use crate::token::ToTokens;
use crate::{Parse, Span, TokenStream};

#[doc = "A whole parsed crate (inner attributes + items)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Crate {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub items: Vec<Item>,
}

impl Parse for Crate {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse_vec::<Attribute>()?;
        let items = stream.parse_vec::<Item>()?;
        Ok(Self {
            span: Span::default(),
            attrs,
            items,
        })
    }
}

impl ToTokens for Crate {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        for it in &self.items {
            it.to_tokens(t);
        }
    }
}
