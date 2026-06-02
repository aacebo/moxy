use crate::ast::Attribute;
use crate::token::ToTokens;
use crate::token::punct::Eq;
use crate::{Span, TokenStream};

#[doc = "An assignment expression: `a = b`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprAssign {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub left: Box<super::super::Expr>,
    pub right: Box<super::super::Expr>,
}

impl ToTokens for ExprAssign {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.left.to_tokens(t);
        Eq::default().to_tokens(t);
        self.right.to_tokens(t);
    }
}
