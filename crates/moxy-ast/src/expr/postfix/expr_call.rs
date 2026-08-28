use moxy_token::Token;
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A function call expression: `f(a, b)`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprCall {
    pub attrs: Attributes,
    pub func: Box<Expr>,
    pub args: Delimited<Punctuated<Expr, Token![,]>>,
}

impl Spanner for ExprCall {
    fn span(&self) -> Span {
        self.attrs.span().join(self.args.span())
    }
}

impl ToTokens for ExprCall {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.func.to_tokens(t);
        self.args.to_tokens(t);
    }
}

impl ExprCall {
    pub fn into_postfix_expr(self) -> super::PostfixExpr {
        super::PostfixExpr::from(self)
    }
}
