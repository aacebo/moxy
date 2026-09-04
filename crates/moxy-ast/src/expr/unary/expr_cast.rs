use crate::Token;
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A cast expression: `x as u32`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprCast {
    pub attrs: Attributes,
    pub expr: Box<Expr>,
    pub as_keyword: Token![as],
    pub ty: Box<Type>,
}

impl Spanner for ExprCast {
    fn span(&self) -> Span {
        self.attrs.span().join(self.ty.span())
    }
}

impl ToTokens for ExprCast {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.expr.to_tokens(t);
        self.as_keyword.to_tokens(t);
        self.ty.to_tokens(t);
    }
}

impl ExprCast {
    pub fn into_unary_expr(self) -> super::UnaryExpr {
        super::UnaryExpr::from(self)
    }
}
