use crate::ast::*;
use crate::token::ToTokens;
use crate::token::keyword::While;
use crate::{Span, TokenStream};

#[doc = "A while loop expression: `while cond { ... }`, `while let pat = expr { ... }`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprWhile {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub label: Option<Label>,
    pub cond: Box<super::super::Expr>,
    pub body: StmtBlock,
}

impl ToTokens for ExprWhile {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }

        if let Some(l) = &self.label {
            l.to_tokens(t);
        }

        While::default().to_tokens(t);
        self.cond.to_tokens(t);
        self.body.to_tokens(t);
    }
}
