use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A parenthesized expression: `(x + y)`.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprParen {
    pub attrs: Attributes,
    pub content: Delimited<Box<Expr>>,
}

impl From<ExprParen> for Expr {
    fn from(value: ExprParen) -> Self {
        Self::Paren(value)
    }
}

impl Spanner for ExprParen {
    fn span(&self) -> Span {
        self.attrs.span().join(self.content.span())
    }
}

impl ToTokens for ExprParen {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.content.to_tokens(t);
    }
}
