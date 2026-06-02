use crate::ast::Attribute;
use crate::token::ToTokens;
use crate::token::keyword::Yield;
use crate::{Span, TokenStream};

#[doc = "A yield expression: `yield`, `yield expr`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprYield {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub expr: Option<Box<super::super::Expr>>,
}

impl ToTokens for ExprYield {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        Yield::default().to_tokens(t);

        if let Some(e) = &self.expr {
            e.to_tokens(t);
        }
    }
}
