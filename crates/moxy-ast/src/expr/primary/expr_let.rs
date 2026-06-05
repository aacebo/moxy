use moxy_token::keyword::Let;
use moxy_token::punct::Eq;
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

#[doc = "A let guard expression used in `if let` / `while let`: `let pat = expr`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprLet {
    pub attrs: Vec<Attribute>,
    pub let_keyword: Let,
    pub pat: Box<Pattern>,
    pub eq: Eq,
    pub expr: Box<super::super::Expr>,
}

impl Spanner for ExprLet {
    fn span(&self) -> Span {
        let start = if let Some(a) = self.attrs.first() {
            a.span()
        } else {
            self.let_keyword.span()
        };
        start.join(self.expr.span())
    }
}

impl ToTokens for ExprLet {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.let_keyword.to_tokens(t);
        self.pat.to_tokens(t);
        self.eq.to_tokens(t);
        self.expr.to_tokens(t);
    }
}
