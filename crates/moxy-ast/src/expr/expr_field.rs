use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A field access expression: `x.field`, `tuple.0`.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprField {
    pub attrs: Attributes,
    pub base: Box<Expr>,
    pub dot: Token![.],
    pub member: Member,
}

impl From<ExprField> for Expr {
    fn from(value: ExprField) -> Self {
        Self::Field(value)
    }
}

impl Spanner for ExprField {
    fn span(&self) -> Span {
        self.attrs.span().join(self.member.span())
    }
}

impl ToTokens for ExprField {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.base.to_tokens(t);
        self.dot.to_tokens(t);
        self.member.to_tokens(t);
    }
}
