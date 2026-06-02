use super::Lit;
use crate::parse::{ParseError, ParseStream};
use crate::token::{self, LexError, ToTokens};
use crate::{Parse, Span, TokenStream};

#[doc = "A character literal (`'a'`, `'\\n'`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct LitChar {
    pub span: Span,
    pub repr: String,
}

impl Parse for LitChar {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let at = stream.span();

        match Lit::parse(stream)? {
            Lit::Char(v) => Ok(v),
            _ => Err(LexError::new(at).message("expected char literal").into()),
        }
    }
}

impl ToTokens for LitChar {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        token::Literal::from_repr(&self.repr, self.span).to_tokens(tokens);
    }
}

impl std::fmt::Display for LitChar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.repr)
    }
}
