use moxy_token::{Span, Spanner, ToTokenStream, ToTokens, TokenStream};

use crate::*;

/// An if expression: `if cond { .. } else if cond { .. } else { .. }`.
///
/// The `else if` branches are held in order in one node, so every traversal
/// of a long chain of them runs without recursing once per branch.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprIfChain {
    pub attrs: Attributes,
    pub if_keyword: Token![if],
    pub cond: Box<Expr>,
    pub then_branch: StmtBlock,
    pub else_ifs: Vec<ElseIf>,
    pub else_keyword: Option<Token![else]>,
    pub else_branch: Option<StmtBlock>,
}

impl Attributed for ExprIfChain {
    fn attrs(&self) -> &[Attribute] {
        &self.attrs
    }
}

impl From<ExprIfChain> for Expr {
    fn from(value: ExprIfChain) -> Self {
        Self::IfChain(value)
    }
}

impl Spanner for ExprIfChain {
    fn span(&self) -> Span {
        self.to_token_stream().span()
    }
}

impl ToTokens for ExprIfChain {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.if_keyword.to_tokens(t);
        self.cond.to_tokens(t);
        self.then_branch.to_tokens(t);

        for branch in &self.else_ifs {
            branch.to_tokens(t);
        }

        self.else_keyword.to_tokens(t);
        self.else_branch.to_tokens(t);
    }
}

/// An `else if cond { .. }` branch of an [`ExprIfChain`].
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ElseIf {
    pub else_keyword: Token![else],
    pub if_keyword: Token![if],
    pub cond: Box<Expr>,
    pub then_branch: StmtBlock,
}

impl Spanner for ElseIf {
    fn span(&self) -> Span {
        self.to_token_stream().span()
    }
}

impl ToTokens for ElseIf {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.else_keyword.to_tokens(t);
        self.if_keyword.to_tokens(t);
        self.cond.to_tokens(t);
        self.then_branch.to_tokens(t);
    }
}
