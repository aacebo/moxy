use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A raw pointer type (e.g. `*const T`, `*mut T`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TypePointer {
    pub star: Token![*],
    pub mutability: PointerMutability,
    pub elem: Box<Type>,
}

impl Parse for TypePointer {
    fn peek(cursor: Cursor<'_>) -> bool {
        <Token![*]>::peek(cursor) && PointerMutability::peek(cursor.offset(1))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            star: parser.parse()?,
            mutability: parser.parse()?,
            elem: Box::new(parser.parse()?),
        })
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Type::skip(PointerMutability::skip(<Token![*]>::skip(cursor)?)?)
    }
}

impl Spanner for TypePointer {
    fn span(&self) -> Span {
        self.star.span().join(self.elem.span())
    }
}

impl ToTokens for TypePointer {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.star.to_tokens(tokens);
        self.mutability.to_tokens(tokens);
        self.elem.to_tokens(tokens);
    }
}
