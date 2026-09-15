use moxy_token::{Keyword, LexError, Quote, Span, Spanner, ToTokens, TokenStream, TokenTree};

use crate::*;

/// A named lifetime (e.g. `'a`, `'static`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Lifetime {
    pub quote: Quote,
    pub ident: LifetimeName,
}

impl Parse for Lifetime {
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.peek::<Quote>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            quote: parser.parse()?,
            ident: parser.parse()?,
        })
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

/// The name part of a lifetime (e.g. the `a` in `'a`, or the `static` in `'static`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct LifetimeName {
    pub span: Span,
    pub text: String,
    pub raw: bool,
}

impl Parse for LifetimeName {
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.peek::<Ident>() || cursor.peek::<Keyword>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        if parser.peek::<Ident>() {
            let token = parser.parse::<Ident>()?;

            return Ok(Self {
                span: token.span(),
                text: token.text().to_string(),
                raw: token.is_raw(),
            });
        }

        if parser.peek::<Keyword>() {
            let token = parser.parse::<Keyword>()?;

            return Ok(Self {
                span: token.span(),
                text: token.text().to_string(),
                raw: false,
            });
        }

        parser.error("expected lifetime name").into()
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
