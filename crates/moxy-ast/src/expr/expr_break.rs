use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A break expression: `break`, `break 'label`, `break expr`.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprBreak {
    pub attrs: Attributes,
    pub break_keyword: Token![break],
    pub label: Option<Label>,
    pub expr: Option<Box<Expr>>,
}

impl From<ExprBreak> for Expr {
    fn from(value: ExprBreak) -> Self {
        Self::Break(value)
    }
}

impl Spanner for ExprBreak {
    fn span(&self) -> Span {
        let end = if let Some(e) = &self.expr {
            e.span()
        } else if let Some(l) = &self.label {
            l.span()
        } else {
            self.break_keyword.span()
        };

        self.attrs.span().join(end)
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
