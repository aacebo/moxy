use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// `&raw const place` or `&raw mut place`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprRawAddr {
    pub attrs: Attributes,
    pub and: Token![&],
    pub raw: Token![raw],
    pub mutability: PointerMutability,
    pub expr: Box<Expr>,
}

impl From<ExprRawAddr> for Expr {
    fn from(value: ExprRawAddr) -> Self {
        Self::RawAddr(value)
    }
}

impl Spanner for ExprRawAddr {
    fn span(&self) -> Span {
        self.attrs.span().join(self.expr.span())
    }
}

impl ToTokens for ExprRawAddr {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.and.to_tokens(t);
        self.mutability.to_tokens(t);
        self.expr.to_tokens(t);
    }
}
