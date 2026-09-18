use moxy_token::{Delim, Span, Spanner, ToTokens, TokenStream};

use super::Type;
use crate::*;

/// A fixed-size array type (`[T; N]`).
#[derive(Debug, Clone, PartialEq, Eq)]
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
        let inner = inner.skip::<ArrayInner>()?;
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

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ArrayInner {
    pub elem: Box<Type>,
    pub semi: Token![;],
    pub len: Expr,
}

impl Parse for ArrayInner {
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.peek::<Type>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let elem = parser.parse()?;
        let semi = parser.parse()?;
        let len = parser.parse()?;
        Ok(Self { elem, semi, len })
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor.skip::<Type>()?.skip::<Token![;]>()?.skip::<Expr>()
    }
}

impl ToTokens for ArrayInner {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.elem.to_tokens(t);
        self.semi.to_tokens(t);
        self.len.to_tokens(t);
    }
}
