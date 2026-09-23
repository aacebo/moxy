use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// An assignment expression: `a = b`.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprAssign {
    pub attrs: Attributes,
    pub left: Box<Expr>,
    pub eq: Token![=],
    pub right: Box<Expr>,
}

impl From<ExprAssign> for Expr {
    fn from(value: ExprAssign) -> Self {
        Self::Assign(value)
    }
}

impl Spanner for ExprAssign {
    fn span(&self) -> Span {
        self.attrs.span().join(self.right.span())
    }
}

impl ToTokens for ExprAssign {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.left.to_tokens(t);
        self.eq.to_tokens(t);
        self.right.to_tokens(t);
    }
}
