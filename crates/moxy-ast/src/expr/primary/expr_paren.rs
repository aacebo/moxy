use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A parenthesized expression: `(x + y)`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprParen {
    pub attrs: Attributes,
    pub content: Delimited<Box<Expr>>,
}

impl Spanner for ExprParen {
    fn span(&self) -> Span {
        self.content.span()
    }
}

impl ToTokens for ExprParen {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.content.to_tokens(t);
    }
}

impl ExprParen {
    pub fn into_primary_expr(self) -> super::PrimaryExpr {
        super::PrimaryExpr::from(self)
    }
}
