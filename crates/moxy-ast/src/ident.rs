use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::{LexError, Parse, Span, Spanner, ToTokens, Token, TokenStream, TokenTree};

/// An identifier token (e.g. a variable name, type name, or keyword-like ident).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Ident {
    pub span: Span,
    pub text: String,
    pub raw: bool,
}

impl Parse for Ident {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let at = stream.span();

        match stream.advance() {
            Some(TokenTree::Token(Token::Ident(id))) => {
                let name = id.text();
                let (raw, text) = match name.strip_prefix("r#") {
                    Some(rest) => (true, rest.to_string()),
                    None => (false, name.into_owned()),
                };

                Ok(Self {
                    span: id.span(),
                    text,
                    raw,
                })
            }
            _ => Err(LexError::new(at).message("expected identifier").into()),
        }
    }
}

impl Spanner for Ident {
    fn span(&self) -> Span {
        self.span
    }
}

impl ToTokens for Ident {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = if self.raw {
            format!("r#{}", self.text)
        } else {
            self.text.clone()
        };

        moxy_token::Ident::new(&name).with_span(self.span).to_tokens(tokens);
    }
}

impl std::fmt::Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.raw {
            write!(f, "r#{}", self.text)
        } else {
            f.write_str(&self.text)
        }
    }
}

#[cfg(test)]
mod tests {
    use moxy_token::ToTokenStream;

    use super::*;

    #[test]
    fn plain() {
        let id = moxy_token::parse!("foo" as Ident).unwrap();
        assert_eq!(id.text, "foo");
        assert!(!id.raw);
        assert_eq!(id.to_token_stream().to_string(), "foo");
    }

    #[test]
    fn raw() {
        let id = moxy_token::parse!("r#fn" as Ident).unwrap();
        assert_eq!(id.text, "fn");
        assert!(id.raw);
        assert_eq!(id.to_token_stream().to_string(), "r#fn");
    }

    #[test]
    fn not_an_ident() {
        assert!(moxy_token::parse!("+" as Ident).is_err());
    }
}
