use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// An index expression: `a[0]`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprIndex {
    pub attrs: Attributes,
    pub base: Box<Expr>,
    pub index: Delimited<Box<Expr>>,
}

impl Spanner for ExprIndex {
    fn span(&self) -> Span {
        let start = if let Some(a) = self.attrs.first() {
            a.span()
        } else {
            self.base.span()
        };
        start.join(self.index.span())
    }
}

impl ToTokens for ExprIndex {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.base.to_tokens(t);
        self.index.to_tokens(t);
    }
}

impl ExprIndex {
    pub fn into_postfix_expr(self) -> super::PostfixExpr {
        super::PostfixExpr::from(self)
    }
}
