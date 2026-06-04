use moxy_token::parse::{ParseError, ParseStream};
use moxy_token::punct::DotDotDot;
use moxy_token::{Parse, Span, ToTokens, TokenStream};

use crate::{Attribute, Ident};

#[doc = "A C-style variadic marker (`...`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Variadic {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub name: Option<Ident>,
    pub dots: DotDotDot,
}

impl Parse for Variadic {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse::<Vec<Attribute>>()?;
        let dots = stream.parse::<DotDotDot>()?;
        Ok(Self {
            span: Span::default(),
            attrs,
            name: None,
            dots,
        })
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
