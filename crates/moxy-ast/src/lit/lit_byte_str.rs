use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::{LexError, Parse, Span, Spanner, ToTokens, TokenStream};

use super::Lit;

/// A byte string literal (`b"hello"`).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct LitByteStr {
    pub span: Span,
    pub repr: String,
}

impl Parse for LitByteStr {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let at = stream.span();

        match Lit::parse(stream)? {
            Lit::ByteStr(v) => Ok(v),
            _ => Err(LexError::new(at).message("expected byte-string literal").into()),
        }
    }
}

impl Spanner for LitByteStr {
    fn span(&self) -> Span {
        self.span
    }
}

impl ToTokens for LitByteStr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        moxy_token::Literal::from_repr(&self.repr, self.span).to_tokens(tokens);
    }
}

impl std::fmt::Display for LitByteStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.repr)
    }
}
