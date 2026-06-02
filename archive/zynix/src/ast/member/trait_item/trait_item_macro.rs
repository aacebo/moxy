use super::TraitItem;
use crate::ast::{Attribute, MacroCall};
use crate::parse::{ParseError, ParseStream};
use crate::token::ToTokens;
use crate::token::punct::Semi;
use crate::{Parse, Span, TokenStream};

#[doc = "A macro invocation inside a trait definition."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TraitItemMacro {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub mac: MacroCall,
    pub semi: bool,
}

impl Parse for TraitItemMacro {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse_vec::<Attribute>()?;
        let (mac, semi) = crate::ast::MacroCall::parse_semi(stream)?;
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

        if self.semi {
            Semi::default().to_tokens(t);
        }
    }
}
