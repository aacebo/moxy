use moxy_token::Token;
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::{Parse, Span, Spanner, ToTokens, TokenStream};

use crate::expr::parse_expr;
use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct RepeatInner {
    pub elem: Box<Expr>,
    pub semi: Token![;],
    pub len: Box<Expr>,
}

impl Parse for RepeatInner {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let elem = Box::new(parse_expr(stream, true)?);
        let semi = stream.parse::<Token![;]>()?;
        let len = Box::new(parse_expr(stream, true)?);
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

/// A repeat expression: `[0u8; 16]`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprRepeat {
    pub attrs: Attributes,
    pub content: Delimited<RepeatInner>,
}

impl Spanner for ExprRepeat {
    fn span(&self) -> Span {
        self.content.span()
    }
}

impl ToTokens for ExprRepeat {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.content.to_tokens(t);
    }
}

impl ExprRepeat {
    pub fn into_primary_expr(self) -> super::PrimaryExpr {
        super::PrimaryExpr::from(self)
    }
}
