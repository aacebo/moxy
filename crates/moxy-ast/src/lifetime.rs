use moxy_token::Token;
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::Quote;
use moxy_token::{LexError, Parse, Span, Spanner, ToTokens, TokenStream, TokenTree};

/// A named lifetime (e.g. `'a`, `'static`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Lifetime {
    pub quote: Quote,
    pub ident: LifetimeName,
}

impl Parse for Lifetime {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let quote = stream.parse::<Quote>()?;
        let ident = stream.parse::<LifetimeName>()?;
        Ok(Self { quote, ident })
    }
}

impl Spanner for Lifetime {
    fn span(&self) -> Span {
        self.quote.span().join(self.ident.span())
    }
}

impl ToTokens for Lifetime {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.quote.to_tokens(tokens);
        self.ident.to_tokens(tokens);
    }
}

impl Lifetime {
    pub fn parse_bounds(
        stream: &mut moxy_token::parser::ParseStream,
    ) -> Result<crate::Punctuated<Self, Token![+]>, moxy_token::parser::ParseError> {
        let mut bounds = crate::Punctuated::new();
        if stream.peek::<Token![:]>() {
            let _ = stream.parse::<Token![:]>()?;

            loop {
                bounds.push_value(stream.parse::<Self>()?);

                if stream.peek::<Token![+]>() {
                    bounds.push_punct(stream.parse::<Token![+]>()?);
                } else {
                    break;
                }
            }
        }
        Ok(bounds)
    }
}

/// The name part of a lifetime (e.g. the `a` in `'a`, or the `static` in `'static`).
#[derive(Debug, Clone, PartialEq, Eq)]
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

        moxy_token::Ident::new(&name).with_span(self.span).to_tokens(tokens);
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
