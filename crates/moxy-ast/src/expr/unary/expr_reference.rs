use crate::Token;
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A reference expression: `&x`, `&mut x`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprReference {
    pub attrs: Attributes,
    pub and_punct: Token![&],
    pub mutability: Mutability,
    pub expr: Box<Expr>,
}

impl Spanner for ExprReference {
    fn span(&self) -> Span {
        self.attrs.span().join(self.expr.span())
    }
}

impl ToTokens for ExprReference {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.and_punct.to_tokens(t);
        self.mutability.to_tokens(t);
        self.expr.to_tokens(t);
    }
}

impl ExprReference {
    pub fn into_unary_expr(self) -> super::UnaryExpr {
        super::UnaryExpr::from(self)
    }
}
