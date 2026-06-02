use moxy_token::punct::And;
use moxy_token::{Span, ToTokens, TokenStream};

use crate::*;

#[doc = "A reference expression: `&x`, `&mut x`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprReference {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub and_punct: And,
    pub mutability: Mutability,
    pub expr: Box<super::super::Expr>,
}

impl ToTokens for ExprReference {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.and_punct.to_tokens(t);
        self.mutability.to_tokens(t);
        self.expr.to_tokens(t);
    }
}
