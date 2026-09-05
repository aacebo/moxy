use crate::Token;
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A let guard expression used in `if let` / `while let`: `let pat = expr`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprLet {
    pub attrs: Attributes,
    pub let_keyword: Token![let],
    pub pat: Box<Pattern>,
    pub eq: Token![=],
    pub expr: Box<Expr>,
}

impl Spanner for ExprLet {
    fn span(&self) -> Span {
        self.attrs.span().join(self.expr.span())
    }
}

impl ToTokens for ExprLet {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.let_keyword.to_tokens(t);
        self.pat.to_tokens(t);
        self.eq.to_tokens(t);
        self.expr.to_tokens(t);
    }
}

impl ExprLet {
    pub fn into_primary_expr(self) -> super::PrimaryExpr {
        super::PrimaryExpr::from(self)
    }
}
