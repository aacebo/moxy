use crate::Token;
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A type ascription expression: `expr: Type`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprType {
    pub attrs: Attributes,
    pub expr: Box<Expr>,
    pub colon_punct: Token![:],
    pub ty: Box<Type>,
}

impl Spanner for ExprType {
    fn span(&self) -> Span {
        self.attrs.span().join(self.ty.span())
    }
}

impl ToTokens for ExprType {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
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
