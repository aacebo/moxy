use moxy_token::punct::And;
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A reference expression: `&x`, `&mut x`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprReference {
    pub attrs: Vec<Attribute>,
    pub and_punct: And,
    pub mutability: Mutability,
    pub expr: Box<Expr>,
}

impl Spanner for ExprReference {
    fn span(&self) -> Span {
        let start = if let Some(a) = self.attrs.first() {
            a.span()
        } else {
            self.and_punct.span()
        };
        start.join(self.expr.span())
    }
}

impl ToTokens for ExprReference {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
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
