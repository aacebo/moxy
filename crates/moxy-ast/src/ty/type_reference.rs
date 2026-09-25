use crate::{Cursor, Parse, ParseError, Parser};
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use super::Type;
use crate::Lifetime;

/// A reference type (e.g. `&'a T`, `&mut T`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TypeReference {
    pub and: Token![&],
    pub lifetime: Option<Lifetime>,
    pub mutability: Option<Token![mut]>,
    pub elem: Box<Type>,
}

impl Parse for TypeReference {
    fn peek(cursor: Cursor<'_>) -> bool {
        <Token![&]>::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let and = <_ as Parse>::parse(parser)?;
        let lifetime = <_ as Parse>::parse(parser)?;
        let mutability = <_ as Parse>::parse(parser)?;
        let elem = Box::new(<_ as Parse>::parse(parser)?);

        Ok(Self {
            and,
            lifetime,
            mutability,
            elem,
        })
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Type::skip(Option::<Token![mut]>::skip(Option::<Lifetime>::skip(<Token![&]>::skip(
            cursor,
        )?)?)?)
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
