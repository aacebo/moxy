use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::{LexError, Parse, Span, Spanner, ToTokens, TokenStream};

use super::Lit;

/// A floating-point literal (`1.0`, `3.14f64`).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct LitFloat {
    pub span: Span,
    pub repr: String,
}

impl Parse for LitFloat {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let at = stream.span();

        match Lit::parse(stream)? {
            Lit::Float(v) => Ok(v),
            _ => Err(LexError::new(at).message("expected float literal").into()),
        }
    }
}

impl Spanner for LitFloat {
    fn span(&self) -> Span {
        self.span
    }
}

impl ToTokens for LitFloat {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        moxy_token::Literal::from_repr(&self.repr, self.span).to_tokens(tokens);
    }
}

impl std::fmt::Display for LitFloat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.repr)
    }
}
