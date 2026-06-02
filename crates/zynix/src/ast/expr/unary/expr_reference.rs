use crate::ast::*;
use crate::token::ToTokens;
use crate::token::punct::And;
use crate::{Span, TokenStream};

#[doc = "A reference expression: `&x`, `&mut x`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprReference {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub mutability: Mutability,
    pub expr: Box<super::super::Expr>,
}

impl ToTokens for ExprReference {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        And::default().to_tokens(t);
        self.mutability.to_tokens(t);
        self.expr.to_tokens(t);
    }
}
