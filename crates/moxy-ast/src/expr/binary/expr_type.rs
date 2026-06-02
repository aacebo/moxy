use moxy_token::punct::Colon;
use moxy_token::{Span, ToTokens, TokenStream};

use crate::*;

#[doc = "A type ascription expression: `expr: Type`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprType {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub expr: Box<super::super::Expr>,
    pub ty: Box<Type>,
}

impl ToTokens for ExprType {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.expr.to_tokens(t);
        Colon::default().to_tokens(t);
        self.ty.to_tokens(t);
    }
}
