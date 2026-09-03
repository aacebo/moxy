use crate::{Parse, ParseError, Parser};
use moxy_token::Token;
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::{Attributes, Ident};

/// A C-style variadic marker (`...`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Variadic {
    pub attrs: Attributes,
    pub name: Option<Ident>,
    pub dots: Token![...],
}

impl Parse for Variadic {
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = parser.parse::<Attributes>()?;
        let dots = parser.parse::<Token![...]>()?;
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
