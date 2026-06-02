use crate::ast::*;
use crate::token::ToTokens;
use crate::{Span, TokenStream};

#[doc = "A compound assignment expression: `a += b`, `x >>= y`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprAssignOp {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub left: Box<super::super::Expr>,
    pub op: AssignOp,
    pub right: Box<super::super::Expr>,
}

impl ToTokens for ExprAssignOp {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.left.to_tokens(t);
        self.op.to_tokens(t);
        self.right.to_tokens(t);
    }
}
