use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A binary operation expression: `a + b`, `x && y`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprBinary {
    pub attrs: Attributes,
    pub left: Box<Expr>,
    pub op: BinOp,
    pub right: Box<Expr>,
}

impl Spanner for ExprBinary {
    fn span(&self) -> Span {
        self.attrs.span().join(self.right.span())
    }
}

impl ToTokens for ExprBinary {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.left.to_tokens(t);
        self.op.to_tokens(t);
        self.right.to_tokens(t);
    }
}

impl ExprBinary {
    pub fn into_binary_expr(self) -> super::BinaryExpr {
        super::BinaryExpr::from(self)
    }
}
