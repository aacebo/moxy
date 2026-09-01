use moxy_token::Token;
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::{Parse, Span, Spanner, ToTokens, TokenStream};

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
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse::<Attributes>()?;
        let vis = stream.parse::<Visibility>()?;
        let mutability = stream.parse::<Mutability>()?;
        let (ident, colon) = if stream.peek::<Ident>() {
            let mut fork = stream.lookahead();
            fork.advance();

            if fork.peek::<Token![:]>() {
                (Some(stream.parse()?), Some(stream.parse()?))
            } else {
                (None, None)
            }
        } else {
            (None, None)
        };

        let ty = stream.parse::<Type>()?;

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
