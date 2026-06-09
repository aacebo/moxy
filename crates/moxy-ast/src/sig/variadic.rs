use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::DotDotDot;
use moxy_token::{Parse, Span, Spanner, ToTokens, TokenStream};

use crate::{Attribute, Ident};

/// A C-style variadic marker (`...`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Variadic {
    pub attrs: Vec<Attribute>,
    pub name: Option<Ident>,
    pub dots: DotDotDot,
}

impl Parse for Variadic {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse::<Vec<Attribute>>()?;
        let dots = stream.parse::<DotDotDot>()?;
        Ok(Self { attrs, name: None, dots })
    }
}

impl Spanner for Variadic {
    fn span(&self) -> Span {
        let start = if let Some(a) = self.attrs.first() {
            a.span()
        } else {
            self.dots.span()
        };
        start.join(self.dots.span())
    }
}

impl ToTokens for Variadic {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.dots.to_tokens(t);
    }
}
