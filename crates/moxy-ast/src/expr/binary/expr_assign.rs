use moxy_token::punct::Eq;
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::{Attributes, Expr};

/// An assignment expression: `a = b`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprAssign {
    pub attrs: Attributes,
    pub left: Box<Expr>,
    pub eq: Eq,
    pub right: Box<Expr>,
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

impl ExprAssign {
    pub fn into_binary_expr(self) -> super::BinaryExpr {
        super::BinaryExpr::from(self)
    }
}
