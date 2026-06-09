use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::{LexError, Parse, Span, Spanner, ToTokens, TokenStream};

use super::Lit;

/// A byte literal (`b'a'`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct LitByte {
    pub span: Span,
    pub repr: String,
}

impl Parse for LitByte {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let at = stream.span();

        match Lit::parse(stream)? {
            Lit::Byte(v) => Ok(v),
            _ => Err(LexError::new(at).message("expected byte literal").into()),
        }
    }
}

impl Spanner for LitByte {
    fn span(&self) -> Span {
        self.span
    }
}

impl ToTokens for LitByte {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        moxy_token::Literal::from_repr(&self.repr, self.span).to_tokens(tokens);
    }
}

impl std::fmt::Display for LitByte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.repr)
    }
}

impl LitByte {
    pub fn into_lit(self) -> super::Lit {
        super::Lit::from(self)
    }
}
