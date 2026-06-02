use moxy_token::parse::{ParseError, ParseStream};
use moxy_token::punct::Semi;
use moxy_token::{Parse, Span, ToTokens, TokenStream};

use crate::{Attribute, Ident, MacroCall};

#[doc = "A macro invocation used as an item (`name!(...);`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemMacro {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub ident: Option<Ident>,
    pub mac: MacroCall,
    pub semi: bool,
    pub semi_punct: Option<Semi>,
}

impl Parse for ItemMacro {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse::<Vec<Attribute>>()?;
        let mac = stream.parse::<MacroCall>()?;

        let (semi, semi_punct) = if stream.peek::<Semi>().is_some() {
            let punct = stream.parse::<Semi>()?;
            (true, Some(punct))
        } else {
            (false, None)
        };

        Ok(ItemMacro {
            span: Span::default(),
            attrs,
            ident: None,
            mac,
            semi,
            semi_punct,
        })
    }
}

impl ToTokens for ItemMacro {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.mac.to_tokens(t);

        if let Some(semi_punct) = &self.semi_punct {
            semi_punct.to_tokens(t);
        }
    }
}
