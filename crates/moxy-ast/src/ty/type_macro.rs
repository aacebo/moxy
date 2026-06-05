use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::{Parse, Span, Spanner, ToTokens, TokenStream};

use crate::MacroCall;

#[doc = "A macro invocation in type position (`path!(...)`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TypeMacro {
    pub mac: MacroCall,
}

impl Parse for TypeMacro {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        Ok(Self {
            mac: stream.parse::<MacroCall>()?,
        })
    }
}

impl Spanner for TypeMacro {
    fn span(&self) -> Span {
        self.mac.span()
    }
}

impl ToTokens for TypeMacro {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.mac.to_tokens(tokens);
    }
}
