use moxy_token::keyword::Return;
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A return expression: `return`, `return expr`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprReturn {
    pub attrs: Attributes,
    pub return_keyword: Return,
    pub expr: Option<Box<Expr>>,
}

impl Spanner for ExprReturn {
    fn span(&self) -> Span {
        let start = if let Some(a) = self.attrs.first() {
            a.span()
        } else {
            self.return_keyword.span()
        };
        let end = if let Some(e) = &self.expr {
            e.span()
        } else {
            self.return_keyword.span()
        };
        start.join(end)
    }
}

impl ToTokens for ExprReturn {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
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
