use crate::parse::{ParseError, ParseStream};
use crate::token::{self, LexError, ToTokens};
use crate::{Parse, Span, Token, TokenStream, TokenTree};

#[doc = "The name part of a lifetime (e.g. the `a` in `'a`, or the `static` in `'static`)."]
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
            Some(TokenTree::Token(Token::Ident(id))) => {
                let name = id.name();
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
            Some(TokenTree::Token(Token::Keyword(kw))) => Ok(Self {
                span: kw.span(),
                text: kw.as_str().to_string(),
                raw: false,
            }),
            _ => Err(LexError::new(at).message("expected lifetime name").into()),
        }
    }
}

impl ToTokens for LifetimeName {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = if self.raw {
            format!("r#{}", self.text)
        } else {
            self.text.clone()
        };

        token::Ident::new(&name, self.span).to_tokens(tokens);
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
