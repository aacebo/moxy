use moxy_token::keyword::As;
use moxy_token::{Span, ToTokens, TokenStream};

use crate::*;

#[doc = "A cast expression: `x as u32`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprCast {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub expr: Box<super::super::Expr>,
    pub as_keyword: As,
    pub ty: Box<Type>,
}

impl ToTokens for ExprCast {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.expr.to_tokens(t);
        self.as_keyword.to_tokens(t);
        self.ty.to_tokens(t);
    }
}
