use crate::ast::MacroCall;
use crate::parse::{ParseError, ParseStream};
use crate::token::ToTokens;
use crate::{Parse, Span, TokenStream};

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
