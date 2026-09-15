use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

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
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.peek::<Token![&]>() || cursor.peek::<Lifetime>() || cursor.peek::<Token![mut]>() || cursor.peek::<Token![self]>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = parser.parse()?;
        let reference = parser.parse()?;
        let lifetime = if reference.is_some() { parser.parse()? } else { None };

        let mutability = parser.parse()?;
        let self_keyword = parser.parse()?;

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
        self.reference.to_tokens(t);
        self.lifetime.to_tokens(t);
        self.mutability.to_tokens(t);
        self.self_keyword.to_tokens(t);
    }
}
