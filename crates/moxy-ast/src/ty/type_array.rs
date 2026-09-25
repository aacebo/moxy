use moxy_token::{Delim, Span, Spanner, ToTokens, TokenStream};

use super::Type;
use crate::*;

/// A fixed-size array type (`[T; N]`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TypeArray {
    pub content: Delimited<ArrayInner>,
}

impl Parse for TypeArray {
    fn peek(cursor: Cursor<'_>) -> bool {
        Self::skip(cursor).is_some()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let content = Delimited::parse_bracket(parser)?;
        Ok(Self { content })
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        let inner = cursor.descend(Delim::Bracket)?;
        let inner = ArrayInner::skip(inner)?;
        inner.is_empty().then(|| cursor.offset(1))
    }
}

impl Spanner for TypeArray {
    fn span(&self) -> Span {
        self.content.span()
    }
}

impl ToTokens for TypeArray {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.content.to_tokens(tokens);
    }
}

/// An AST representation of Rust array inner syntax.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ArrayInner {
    pub elem: Box<Type>,
    pub semi: Token![;],
    pub len: Expr,
}

impl Parse for ArrayInner {
    fn peek(cursor: Cursor<'_>) -> bool {
        Type::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let elem = <_ as Parse>::parse(parser)?;
        let semi = <_ as Parse>::parse(parser)?;
        let len = <_ as Parse>::parse(parser)?;
        Ok(Self { elem, semi, len })
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Expr::skip(<Token![;]>::skip(Type::skip(cursor)?)?)
    }
}

impl ToTokens for ArrayInner {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.elem.to_tokens(t);
        self.semi.to_tokens(t);
        self.len.to_tokens(t);
    }
}
