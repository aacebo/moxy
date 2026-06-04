use moxy_token::parse::{ParseError, ParseStream};
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
    pub bracket: Delimited<ArrayInner>,
}

impl Parse for TypeArray {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let bracket = Delimited::<ArrayInner>::parse_bracket(stream)?;
        Ok(Self {
            span: Span::default(),
            bracket,
        })
    }
}

impl ToTokens for TypeArray {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.bracket.to_tokens(tokens);
    }
}
