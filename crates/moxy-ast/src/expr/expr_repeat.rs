use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A repeat expression: `[0u8; 16]`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprRepeat {
    pub attrs: Attributes,
    pub content: Delimited<RepeatInner>,
}

impl From<ExprRepeat> for Expr {
    fn from(value: ExprRepeat) -> Self {
        Self::Repeat(value)
    }
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

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct RepeatInner {
    pub elem: Box<Expr>,
    pub semi: Token![;],
    pub len: Box<Expr>,
}

impl ToTokens for RepeatInner {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.elem.to_tokens(t);
        self.semi.to_tokens(t);
        self.len.to_tokens(t);
    }
}
