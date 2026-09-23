use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A reference expression: `&x`, `&mut x`.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprReference {
    pub attrs: Attributes,
    pub and: Token![&],
    pub mutability: Mutability,
    pub expr: Box<Expr>,
}

impl From<ExprReference> for Expr {
    fn from(value: ExprReference) -> Self {
        Self::Reference(value)
    }
}

impl Spanner for ExprReference {
    fn span(&self) -> Span {
        self.attrs.span().join(self.expr.span())
    }
}

impl ToTokens for ExprReference {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.and.to_tokens(t);
        self.mutability.to_tokens(t);
        self.expr.to_tokens(t);
    }
}
