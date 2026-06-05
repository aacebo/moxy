use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::Semi;
use moxy_token::{Parse, Span, Spanner, ToTokens, TokenStream};

use crate::{Attribute, Delimited};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct RepeatInner {
    pub elem: Box<super::super::Expr>,
    pub semi: Semi,
    pub len: Box<super::super::Expr>,
}

impl Parse for RepeatInner {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let elem = Box::new(super::super::parse_expr(stream, true)?);
        let semi = stream.parse::<Semi>()?;
        let len = Box::new(super::super::parse_expr(stream, true)?);
        Ok(Self { elem, semi, len })
    }
}

impl ToTokens for RepeatInner {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.elem.to_tokens(t);
        self.semi.to_tokens(t);
        self.len.to_tokens(t);
    }
}

#[doc = "A repeat expression: `[0u8; 16]`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprRepeat {
    pub attrs: Vec<Attribute>,
    pub content: Delimited<RepeatInner>,
}

impl Spanner for ExprRepeat {
    fn span(&self) -> Span {
        self.content.span()
    }
}

impl ToTokens for ExprRepeat {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.content.to_tokens(t);
    }
}
