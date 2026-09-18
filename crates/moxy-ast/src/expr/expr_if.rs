use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// An if expression: `if cond { ... } else { ... }`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprIf {
    pub attrs: Attributes,
    pub if_keyword: Token![if],
    pub cond: Box<Expr>,
    pub then_branch: StmtBlock,
    pub else_keyword: Option<Token![else]>,
    pub else_branch: Option<Box<Expr>>,
}

impl From<ExprIf> for Expr {
    fn from(value: ExprIf) -> Self {
        Self::If(value)
    }
}

impl Spanner for ExprIf {
    fn span(&self) -> Span {
        let end = if let Some(e) = &self.else_branch {
            e.span()
        } else {
            self.then_branch.span()
        };

        self.attrs.span().join(end)
    }
}

impl ToTokens for ExprIf {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.if_keyword.to_tokens(t);
        self.cond.to_tokens(t);
        self.then_branch.to_tokens(t);
        self.else_keyword.to_tokens(t);
        self.else_branch.to_tokens(t);
    }
}
