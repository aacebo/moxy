use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A C-style variadic marker (`...`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Variadic {
    pub attrs: Attributes,
    pub name: Option<Ident>,
    pub dots: Token![...],
}

impl Parse for Variadic {
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.peek::<Token![...]>() || (cursor.peek::<Ident>() && cursor.offset(1).peek::<Token![...]>())
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            attrs: parser.parse()?,
            name: parser.parse()?,
            dots: parser.parse()?,
        })
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
        self.name.to_tokens(t);
        self.dots.to_tokens(t);
    }
}
