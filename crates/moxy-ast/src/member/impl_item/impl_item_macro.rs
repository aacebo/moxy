use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::Semi;
use moxy_token::{Parse, Span, Spanner, ToTokens, TokenStream};

use super::ImplItem;
use crate::{Attribute, MacroCall};

#[doc = "A macro invocation inside an `impl` block."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ImplItemMacro {
    pub attrs: Vec<Attribute>,
    pub mac: MacroCall,
    pub semi: Option<Semi>,
}

impl Parse for ImplItemMacro {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse::<Vec<Attribute>>()?;
        let (mac, semi) = crate::MacroCall::parse_semi(stream)?;
        Ok(ImplItemMacro { attrs, mac, semi })
    }
}

impl Spanner for ImplItemMacro {
    fn span(&self) -> Span {
        let start = if let Some(a) = self.attrs.first() {
            a.span()
        } else {
            self.mac.span()
        };
        let end = self.semi.as_ref().map(|s| s.span()).unwrap_or_else(|| self.mac.span());
        start.join(end)
    }
}

impl ToTokens for ImplItemMacro {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.mac.to_tokens(t);
        self.semi.to_tokens(t);
    }
}
