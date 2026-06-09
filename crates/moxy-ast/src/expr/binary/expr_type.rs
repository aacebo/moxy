use moxy_token::punct::Colon;
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A type ascription expression: `expr: Type`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprType {
    pub attrs: Vec<Attribute>,
    pub expr: Box<Expr>,
    pub colon_punct: Colon,
    pub ty: Box<Type>,
}

impl Spanner for ExprType {
    fn span(&self) -> Span {
        let start = if let Some(a) = self.attrs.first() {
            a.span()
        } else {
            self.expr.span()
        };

        start.join(self.ty.span())
    }
}

impl ToTokens for ExprType {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }

        self.expr.to_tokens(t);
        self.colon_punct.to_tokens(t);
        self.ty.to_tokens(t);
    }
}

impl ExprType {
    pub fn into_binary_expr(self) -> super::BinaryExpr {
        super::BinaryExpr::from(self)
    }
}
