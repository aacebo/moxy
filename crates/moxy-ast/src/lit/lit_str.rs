use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::{LexError, Parse, Span, Spanner, ToTokens, TokenStream};

use super::Lit;

/// A string literal (`"hello"`, `r#"raw"#`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct LitStr {
    pub span: Span,
    pub repr: String,
}

impl Parse for LitStr {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let at = stream.span();

        match Lit::parse(stream)? {
            Lit::Str(v) => Ok(v),
            _ => Err(LexError::new(at).message("expected string literal").into()),
        }
    }
}

impl Spanner for LitStr {
    fn span(&self) -> Span {
        self.span
    }
}

impl ToTokens for LitStr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        moxy_token::Literal::from_repr(&self.repr, self.span).to_tokens(tokens);
    }
}

impl std::fmt::Display for LitStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.repr)
    }
}

impl LitStr {
    pub fn into_lit(self) -> super::Lit {
        super::Lit::from(self)
    }
}
