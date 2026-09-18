use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A unary expression: `!x`, `-x`, `*x`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprUnary {
    pub attrs: Attributes,
    pub op: UnOp,
    pub expr: Box<Expr>,
}

impl From<ExprUnary> for Expr {
    fn from(value: ExprUnary) -> Self {
        Self::Unary(value)
    }
}

impl Spanner for ExprUnary {
    fn span(&self) -> Span {
        self.attrs.span().join(self.expr.span())
    }
}

impl ToTokens for ExprUnary {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.op.to_tokens(t);
        self.expr.to_tokens(t);
    }
}
