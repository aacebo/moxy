use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::Semi;
use moxy_token::{Parse, Span, ToTokens, TokenStream};

use crate::{Attribute, MacroCall};

#[doc = "A macro invocation used as a statement (`name!(...);` or `name!(...)`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct StmtMacro {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub mac: MacroCall,
    pub semi: Option<Semi>,
}

impl Parse for StmtMacro {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse::<Vec<Attribute>>()?;
        let mac = stream.parse::<MacroCall>()?;
        let semi = stream.parse_if::<Semi>();
        Ok(Self {
            span: Span::default(),
            attrs,
            mac,
            semi,
        })
    }
}

impl ToTokens for StmtMacro {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.mac.to_tokens(t);
        self.semi.to_tokens(t);
    }
}
