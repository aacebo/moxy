use moxy_token::parse::{ParseError, ParseStream};
use moxy_token::punct::Semi;
use moxy_token::{Parse, Span, ToTokens, TokenStream};

use super::TraitItem;
use crate::{Attribute, MacroCall};

#[doc = "A macro invocation inside a trait definition."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TraitItemMacro {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub mac: MacroCall,
    pub semi: Option<Semi>,
}

impl Parse for TraitItemMacro {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse::<Vec<Attribute>>()?;
        let (mac, semi) = crate::MacroCall::parse_semi(stream)?;
        Ok(TraitItemMacro {
            span: Span::default(),
            attrs,
            mac,
            semi,
        })
    }
}

impl ToTokens for TraitItemMacro {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.mac.to_tokens(t);
        self.semi.to_tokens(t);
    }
}
