use crate::Token;
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A return expression: `return`, `return expr`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprReturn {
    pub attrs: Attributes,
    pub return_keyword: Token![return],
    pub expr: Option<Box<Expr>>,
}

impl Spanner for ExprReturn {
    fn span(&self) -> Span {
        let end = if let Some(e) = &self.expr {
            e.span()
        } else {
            self.return_keyword.span()
        };
        self.attrs.span().join(end)
    }
}

impl ToTokens for ExprReturn {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.return_keyword.to_tokens(t);

        if let Some(e) = &self.expr {
            e.to_tokens(t);
        }
    }
}

impl ExprReturn {
    pub fn into_jump_expr(self) -> super::JumpExpr {
        super::JumpExpr::from(self)
    }
}
