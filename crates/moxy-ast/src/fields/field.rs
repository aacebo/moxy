use crate::Token;
use crate::{Parse, ParseError, Parser};
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::{Attributes, Ident, Mutability, Type, Visibility};

/// A struct/enum field definition (`pub name: Type` or `pub Type`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Field {
    pub attrs: Attributes,
    pub vis: Visibility,
    pub mutability: Mutability,
    pub ident: Option<Ident>,
    pub colon: Option<Token![:]>,
    pub ty: Type,
}

impl Parse for Field {
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = parser.parse::<Attributes>()?;
        let vis = parser.parse::<Visibility>()?;
        let mutability = parser.parse::<Mutability>()?;
        let (ident, colon) = if parser.peek::<Ident>() {
            let fork = parser.lookahead();
            fork.advance();

            if fork.peek::<Token![:]>() {
                (Some(parser.parse()?), Some(parser.parse()?))
            } else {
                (None, None)
            }
        } else {
            (None, None)
        };

        let ty = parser.parse::<Type>()?;

        Ok(Self {
            attrs,
            vis,
            mutability,
            ident,
            colon,
            ty,
        })
    }
}

impl Spanner for Field {
    fn span(&self) -> Span {
        self.attrs.span().join(self.ty.span())
    }
}

impl ToTokens for Field {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.vis.to_tokens(t);
        self.mutability.to_tokens(t);

        if let Some(id) = &self.ident {
            id.to_tokens(t);
            if let Some(colon) = &self.colon {
                colon.to_tokens(t);
            }
        }

        self.ty.to_tokens(t);
    }
}
