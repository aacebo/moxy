use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::DotDotDot;
use moxy_token::{Parse, Span, Spanner, ToTokens, TokenStream};

use crate::{Attributes, Ident};

/// A C-style variadic marker (`...`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Variadic {
    pub attrs: Attributes,
    pub name: Option<Ident>,
    pub dots: DotDotDot,
}

impl Parse for Variadic {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse::<Attributes>()?;
        let dots = stream.parse::<DotDotDot>()?;
        Ok(Self { attrs, name: None, dots })
    }
}

impl Spanner for Variadic {
    fn span(&self) -> Span {
        self.attrs.span().join(self.dots.span())
    }
}

impl ToTokens for Variadic {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.dots.to_tokens(t);
    }
}
