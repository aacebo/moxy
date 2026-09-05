use crate::{Parse, ParseError, Parser};
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::{Attributes, Ident, Type};

/// An argument of a bare function pointer type.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct BareFnArg {
    pub attrs: Attributes,
    pub name: Option<(Ident, Token![:])>,
    pub ty: Type,
}

impl Parse for BareFnArg {
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = parser.parse::<Attributes>()?;
        let name = if parser.peek::<Ident>() {
            let fork = parser.lookahead();
            fork.advance();

            if fork.peek::<Token![:]>() {
                Some((parser.parse()?, parser.parse()?))
            } else {
                None
            }
        } else {
            None
        };

        let ty = parser.parse::<Type>()?;
        Ok(Self { attrs, name, ty })
    }
}

impl Spanner for BareFnArg {
    fn span(&self) -> Span {
        self.attrs.span().join(self.ty.span())
    }
}

impl ToTokens for BareFnArg {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);

        if let Some((n, colon)) = &self.name {
            n.to_tokens(t);
            colon.to_tokens(t);
        }

        self.ty.to_tokens(t);
    }
}
