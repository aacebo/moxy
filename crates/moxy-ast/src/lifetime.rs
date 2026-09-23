use moxy_token::{Keyword, Quote, Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A named lifetime (e.g. `'a`, `'static`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Lifetime {
    pub quote: Quote,
    pub ident: LifetimeName,
}

impl Parse for Lifetime {
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.peek::<Quote>() && cursor.offset(1).peek::<LifetimeName>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            quote: parser.parse()?,
            ident: parser.parse()?,
        })
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor.skip::<Quote>()?.skip::<LifetimeName>()
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
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
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
                text: token.as_str().to_string(),
                raw: false,
            });
        }

        parser.error("expected lifetime name").into()
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        if cursor.peek::<Ident>() {
            cursor.skip::<Ident>()
        } else {
            cursor.skip::<Keyword>()
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
