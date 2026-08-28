use moxy_token::Token;
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::{Parse, Span, Spanner, ToTokens, TokenStream};

use super::Type;
use crate::{Delimited, Expr};

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ArrayInner {
    pub elem: Box<Type>,
    pub semi: Token![;],
    pub len: Expr,
}

impl Parse for ArrayInner {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let elem = Box::new(stream.parse::<Type>()?);
        let semi = stream.parse::<Token![;]>()?;
        let len = stream.parse::<Expr>()?;
        Ok(Self { elem, semi, len })
    }
}

impl ToTokens for ArrayInner {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.elem.to_tokens(t);
        self.semi.to_tokens(t);
        self.len.to_tokens(t);
    }
}

/// A fixed-size array type (`[T; N]`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TypeArray {
    pub content: Delimited<ArrayInner>,
}

impl Parse for TypeArray {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let content = Delimited::<ArrayInner>::parse_bracket(stream)?;
        Ok(Self { content })
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
