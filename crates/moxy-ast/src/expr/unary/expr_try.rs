use moxy_token::Token;
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A try expression: `expr?`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprTry {
    pub attrs: Attributes,
    pub expr: Box<Expr>,
    pub question_punct: Token![?],
}

impl Spanner for ExprTry {
    fn span(&self) -> Span {
        self.attrs.span().join(self.question_punct.span())
    }
}

impl ToTokens for ExprTry {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.expr.to_tokens(t);
        self.question_punct.to_tokens(t);
    }
}

impl ExprTry {
    pub fn into_unary_expr(self) -> super::UnaryExpr {
        super::UnaryExpr::from(self)
    }
}
