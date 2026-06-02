use super::Lit;
use crate::parse::{ParseError, ParseStream};
use crate::token::{self, LexError, ToTokens};
use crate::{Parse, Span, TokenStream};

#[doc = "A boolean literal (`true` or `false`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct LitBool {
    pub span: Span,
    pub value: bool,
}

impl Parse for LitBool {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let at = stream.span();

        match Lit::parse(stream)? {
            Lit::Bool(v) => Ok(v),
            _ => Err(LexError::new(at).message("expected `true` or `false`").into()),
        }
    }
}

impl ToTokens for LitBool {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let text = if self.value { "true" } else { "false" };
        token::Ident::new(text, self.span).to_tokens(tokens);
    }
}

impl std::fmt::Display for LitBool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(if self.value { "true" } else { "false" })
    }
}
