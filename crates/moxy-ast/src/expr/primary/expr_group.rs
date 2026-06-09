use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A group expression (invisible delimiter wrapper used during macro expansion).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprGroup {
    pub attrs: Vec<Attribute>,
    pub expr: Box<Expr>,
}

impl Spanner for ExprGroup {
    fn span(&self) -> Span {
        let start = self.attrs.first().map(|a| a.span()).unwrap_or_else(|| self.expr.span());
        start.join(self.expr.span())
    }
}

impl ToTokens for ExprGroup {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.expr.to_tokens(t);
    }
}

impl ExprGroup {
    pub fn into_primary_expr(self) -> super::PrimaryExpr {
        super::PrimaryExpr::from(self)
    }
}
