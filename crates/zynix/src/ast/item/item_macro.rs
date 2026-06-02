use crate::ast::{Attribute, Ident, MacroCall};
use crate::parse::{ParseError, ParseStream};
use crate::token::ToTokens;
use crate::token::punct::Semi;
use crate::{Parse, Span, TokenStream};

#[doc = "A macro invocation used as an item (`name!(...);`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemMacro {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub ident: Option<Ident>,
    pub mac: MacroCall,
    pub semi: bool,
}

impl Parse for ItemMacro {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse_vec::<Attribute>()?;
        let mac = stream.parse::<MacroCall>()?;

        let semi = if stream.peek::<Semi>().is_some() {
            let _ = stream.parse::<Semi>()?;
            true
        } else {
            false
        };

        Ok(ItemMacro {
            span: Span::default(),
            attrs,
            ident: None,
            mac,
            semi,
        })
    }
}

impl ToTokens for ItemMacro {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.mac.to_tokens(t);

        if self.semi {
            Semi::default().to_tokens(t);
        }
    }
}
