use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::{LexError, Parse, Span, Spanner, ToTokens, Token, TokenStream, TokenTree};

/// The name part of a lifetime (e.g. the `a` in `'a`, or the `static` in `'static`).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct LifetimeName {
    pub span: Span,
    pub text: String,
    pub raw: bool,
}

impl Parse for LifetimeName {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let at = stream.span();

        // A lifetime name may be an identifier (`'a`) or a keyword (`'static`).
        match stream.advance() {
            Some(TokenTree::Ident(id)) => {
                let (raw, text) = if id.is_raw() {
                    (true, id.text().to_string())
                } else {
                    (false, id.text().to_string())
                };

                Ok(Self {
                    span: id.span(),
                    text,
                    raw,
                })
            }
            Some(TokenTree::Keyword(kw)) => Ok(Self {
                span: kw.span(),
                text: kw.as_str().to_string(),
                raw: false,
            }),
            _ => Err(LexError::new(at).message("expected lifetime name").into()),
        }
    }
}

impl Spanner for LifetimeName {
    fn span(&self) -> Span {
        self.span
    }
}

impl ToTokens for LifetimeName {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = if self.raw {
            format!("r#{}", self.text)
        } else {
            self.text.clone()
        };

        moxy_token::Ident::new(&name, self.span).to_tokens(tokens);
    }
}

impl std::fmt::Display for LifetimeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.raw {
            write!(f, "r#{}", self.text)
        } else {
            f.write_str(&self.text)
        }
    }
}
