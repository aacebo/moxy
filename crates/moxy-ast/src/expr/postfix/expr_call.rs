use moxy_token::punct::Comma;
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A function call expression: `f(a, b)`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprCall {
    pub attrs: Attributes,
    pub func: Box<Expr>,
    pub args: Delimited<Punctuated<Expr, Comma>>,
}

impl Spanner for ExprCall {
    fn span(&self) -> Span {
        let start = if let Some(a) = self.attrs.first() {
            a.span()
        } else {
            self.func.span()
        };
        start.join(self.args.span())
    }
}

impl ToTokens for ExprCall {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.func.to_tokens(t);
        self.args.to_tokens(t);
    }
}

impl ExprCall {
    pub fn into_postfix_expr(self) -> super::PostfixExpr {
        super::PostfixExpr::from(self)
    }
}
