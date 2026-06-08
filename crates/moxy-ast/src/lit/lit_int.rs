use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::{LexError, Parse, Span, Spanner, ToTokens, TokenStream};

use super::Lit;

/// An integer literal (`42`, `0xFF`, `1_000u64`).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct LitInt {
    pub span: Span,
    pub repr: String,
}

impl Parse for LitInt {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let at = stream.span();

        match Lit::parse(stream)? {
            Lit::Int(v) => Ok(v),
            _ => Err(LexError::new(at).message("expected integer literal").into()),
        }
    }
}

impl Spanner for LitInt {
    fn span(&self) -> Span {
        self.span
    }
}

impl ToTokens for LitInt {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        moxy_token::Literal::from_repr(&self.repr, self.span).to_tokens(tokens);
    }
}

impl std::fmt::Display for LitInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.repr)
    }
}
