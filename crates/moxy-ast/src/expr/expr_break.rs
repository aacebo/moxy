use moxy_token::{Span, Spanner, ToTokenStream, ToTokens, TokenStream};

use crate::*;

/// A break expression: `break`, `break 'label`, `break expr`.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprBreak {
    pub attrs: Attributes,
    pub break_keyword: Token![break],
    pub label: Option<Lifetime>,
    pub expr: Option<Box<Expr>>,
}

impl Attributed for ExprBreak {
    fn attrs(&self) -> &[Attribute] {
        &self.attrs
    }
}

impl From<ExprBreak> for Expr {
    fn from(value: ExprBreak) -> Self {
        Self::Break(value)
    }
}

impl Spanner for ExprBreak {
    fn span(&self) -> Span {
        self.to_token_stream().span()
    }
}

impl ToTokens for ExprBreak {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.break_keyword.to_tokens(t);
        self.label.to_tokens(t);
        self.expr.to_tokens(t);
    }
}
