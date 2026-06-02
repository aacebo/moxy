use crate::ast::{Attribute, Ident};
use crate::parse::{ParseError, ParseStream};
use crate::token::ToTokens;
use crate::token::punct::DotDotDot;
use crate::{Parse, Span, TokenStream};

#[doc = "A C-style variadic marker (`...`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Variadic {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub name: Option<Ident>,
}

impl Parse for Variadic {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse_vec::<Attribute>()?;
        let _ = stream.parse::<DotDotDot>()?;
        Ok(Self {
            span: Span::default(),
            attrs,
            name: None,
        })
    }
}

impl ToTokens for Variadic {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        DotDotDot::default().to_tokens(t);
    }
}
