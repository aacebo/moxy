use moxy_token::punct::Comma;
use moxy_token::{Span, ToTokens, TokenStream};

use crate::*;

#[doc = "A function call expression: `f(a, b)`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprCall {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub func: Box<super::super::Expr>,
    pub paren: Delimited<Punctuated<super::super::Expr, Comma>>,
}

impl ToTokens for ExprCall {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.func.to_tokens(t);
        self.paren.to_tokens(t);
    }
}
