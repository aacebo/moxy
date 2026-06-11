use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A compound assignment expression: `a += b`, `x >>= y`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprAssignOp {
    pub attrs: Attributes,
    pub left: Box<Expr>,
    pub op: AssignOp,
    pub right: Box<Expr>,
}

impl Spanner for ExprAssignOp {
    fn span(&self) -> Span {
        let start = if let Some(a) = self.attrs.first() {
            a.span()
        } else {
            self.left.span()
        };

        start.join(self.right.span())
    }
}

impl ToTokens for ExprAssignOp {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.left.to_tokens(t);
        self.op.to_tokens(t);
        self.right.to_tokens(t);
    }
}

impl ExprAssignOp {
    pub fn into_binary_expr(self) -> super::BinaryExpr {
        super::BinaryExpr::from(self)
    }
}
