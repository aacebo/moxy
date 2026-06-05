use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::{Parse, Span, ToTokens, TokenStream};

use crate::MacroCall;

#[doc = "A macro invocation in type position (`path!(...)`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TypeMacro {
    pub span: Span,
    pub mac: MacroCall,
}

impl Parse for TypeMacro {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        Ok(Self {
            span: Span::default(),
            mac: stream.parse::<MacroCall>()?,
        })
    }
}

impl ToTokens for TypeMacro {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.mac.to_tokens(tokens);
    }
}
