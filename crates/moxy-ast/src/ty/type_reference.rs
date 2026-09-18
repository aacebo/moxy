use crate::{Cursor, Parse, ParseError, Parser};
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use super::Type;
use crate::{Lifetime, Mutability};

/// A reference type (e.g. `&'a T`, `&mut T`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TypeReference {
    pub and: Token![&],
    pub lifetime: Option<Lifetime>,
    pub mutability: Mutability,
    pub elem: Box<Type>,
}

impl Parse for TypeReference {
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.peek::<Token![&]>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let and = parser.parse()?;
        let lifetime = parser.parse()?;
        let mutability = parser.parse()?;
        let elem = Box::new(parser.parse()?);

        Ok(Self {
            and,
            lifetime,
            mutability,
            elem,
        })
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor
            .skip::<Token![&]>()?
            .skip::<Option<Lifetime>>()?
            .skip::<Mutability>()?
            .skip::<Type>()
    }
}

impl Spanner for TypeReference {
    fn span(&self) -> Span {
        self.and.span().join(self.elem.span())
    }
}

impl ToTokens for TypeReference {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.and.to_tokens(tokens);
        self.lifetime.to_tokens(tokens);
        self.mutability.to_tokens(tokens);
        self.elem.to_tokens(tokens);
    }
}
