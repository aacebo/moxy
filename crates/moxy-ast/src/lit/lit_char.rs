use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::{LexError, Parse, Span, Spanner, ToTokens, TokenStream};

use super::Lit;

/// A character literal (`'a'`, `'\n'`).
#[derive(Debug, Clone, PartialEq, Eq)]
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

impl Spanner for LitChar {
    fn span(&self) -> Span {
        self.span
    }
}

impl ToTokens for LitChar {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        moxy_token::Literal::from_repr(&self.repr, self.span).to_tokens(tokens);
    }
}

impl std::fmt::Display for LitChar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.repr)
    }
}

impl LitChar {
    pub fn into_lit(self) -> super::Lit {
        super::Lit::from(self)
    }
}
