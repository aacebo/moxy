use crate::{Parse, ParseError, Parser};
use moxy_token::Token;
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::{Attributes, Lifetime, Mutability};

/// A method receiver parameter (`self`, `&self`, `&mut self`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Receiver {
    pub attrs: Attributes,
    pub reference: Option<Token![&]>,
    pub lifetime: Option<Lifetime>,
    pub mutability: Mutability,
    pub self_keyword: Token![self],
}

impl Parse for Receiver {
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = parser.parse::<Attributes>()?;
        let reference = parser.parse_if::<Token![&]>();
        let lifetime = if reference.is_some() {
            parser.parse_if::<Lifetime>()
        } else {
            None
        };

        let mutability = parser.parse::<Mutability>()?;
        let self_keyword = parser.parse::<Token![self]>()?;

        Ok(Self {
            attrs,
            reference,
            lifetime,
            mutability,
            self_keyword,
        })
    }
}

impl Spanner for Receiver {
    fn span(&self) -> Span {
        self.attrs.span().join(self.self_keyword.span())
    }
}

impl ToTokens for Receiver {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);

        if let Some(amp) = &self.reference {
            amp.to_tokens(t);

            if let Some(l) = &self.lifetime {
                l.to_tokens(t);
            }
        }

        self.mutability.to_tokens(t);
        self.self_keyword.to_tokens(t);
    }
}
