use moxy_token::keyword::As;
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A cast expression: `x as u32`.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprCast {
    pub attrs: Vec<Attribute>,
    pub expr: Box<Expr>,
    pub as_keyword: As,
    pub ty: Box<Type>,
}

impl Spanner for ExprCast {
    fn span(&self) -> Span {
        let start = if let Some(a) = self.attrs.first() {
            a.span()
        } else {
            self.expr.span()
        };
        start.join(self.ty.span())
    }
}

impl ToTokens for ExprCast {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
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
