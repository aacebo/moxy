use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::{LexError, Parse, Span, Spanner, ToTokens, TokenStream};

use super::Lit;

/// A C-string literal (`c"hello"` / `c\'\\0\'`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct LitCStr {
    pub span: Span,
    pub repr: String,
}

impl Parse for LitCStr {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let at = stream.span();

        match Lit::parse(stream)? {
            Lit::CStr(v) => Ok(v),
            _ => Err(LexError::new(at).message("expected C-string literal").into()),
        }
    }
}

impl Spanner for LitCStr {
    fn span(&self) -> Span {
        self.span
    }
}

impl ToTokens for LitCStr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        moxy_token::Literal::from_repr(&self.repr, self.span).to_tokens(tokens);
    }
}

impl std::fmt::Display for LitCStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.repr)
    }
}

impl LitCStr {
    pub fn into_lit(self) -> super::Lit {
        super::Lit::from(self)
    }
}
