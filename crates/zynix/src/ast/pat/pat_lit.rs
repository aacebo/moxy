use crate::ast::*;
use crate::token::ToTokens;
use crate::{Span, TokenStream};

#[doc = "A literal pattern, e.g. `42`, `'a'`, or `\"hello\"`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatLit {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub expr: Expr,
}

impl ToTokens for PatLit {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.expr.to_tokens(t);
    }
}
