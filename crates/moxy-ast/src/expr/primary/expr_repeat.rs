use moxy_token::parse::{ParseError, ParseStream};
use moxy_token::punct::Semi;
use moxy_token::{Parse, Span, ToTokens, TokenStream};

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
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub bracket: Delimited<RepeatInner>,
}

impl ToTokens for ExprRepeat {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.bracket.to_tokens(t);
    }
}
