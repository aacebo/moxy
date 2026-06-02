use crate::ast::*;
use crate::token::ToTokens;
use crate::token::keyword::{Else, If};
use crate::{Span, TokenStream};

#[doc = "An if expression: `if cond { ... } else { ... }`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprIf {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub cond: Box<super::super::Expr>,
    pub then_branch: StmtBlock,
    pub else_branch: Option<Box<super::super::Expr>>,
}

impl ToTokens for ExprIf {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        If::default().to_tokens(t);
        self.cond.to_tokens(t);
        self.then_branch.to_tokens(t);

        if let Some(e) = &self.else_branch {
            Else::default().to_tokens(t);
            e.to_tokens(t);
        }
    }
}
