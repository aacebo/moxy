use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// An argument of a bare function pointer type.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct BareFnArg {
    pub attrs: Attributes,
    pub name: Option<(Ident, Token![:])>,
    pub ty: Type,
}

impl Parse for BareFnArg {
    fn peek(cursor: Cursor<'_>) -> bool {
        (cursor.peek::<Ident>() && cursor.offset(1).peek::<Token![:]>()) || cursor.peek::<Type>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = parser.parse()?;
        let name = if parser.peek::<Ident>() {
            Some((parser.parse()?, parser.parse()?))
        } else {
            None
        };

        let colon = parser.parse()?;
        let ty = parser.parse()?;

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
