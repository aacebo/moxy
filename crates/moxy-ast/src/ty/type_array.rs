use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::Semi;
use moxy_token::{Parse, Span, ToTokens, TokenStream};

use super::Type;
use crate::{Delimited, Expr};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ArrayInner {
    pub elem: Box<Type>,
    pub semi: Semi,
    pub len: Expr,
}

impl Parse for ArrayInner {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let elem = Box::new(stream.parse::<Type>()?);
        let semi = stream.parse::<Semi>()?;
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

#[doc = "A fixed-size array type (`[T; N]`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TypeArray {
    pub span: Span,
    pub content: Delimited<ArrayInner>,
}

impl Parse for TypeArray {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let content = Delimited::<ArrayInner>::parse_bracket(stream)?;
        Ok(Self {
            span: Span::default(),
            content,
        })
    }
}

impl ToTokens for TypeArray {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.content.to_tokens(tokens);
    }
}
