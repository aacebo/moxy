use moxy_token::keyword::Let;
use moxy_token::punct::Eq;
use moxy_token::{Span, ToTokens, TokenStream};

use crate::*;

#[doc = "A let guard expression used in `if let` / `while let`: `let pat = expr`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprLet {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub pat: Box<Pattern>,
    pub expr: Box<super::super::Expr>,
}

impl ToTokens for ExprLet {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        Let::default().to_tokens(t);
        self.pat.to_tokens(t);
        Eq::default().to_tokens(t);
        self.expr.to_tokens(t);
    }
}
