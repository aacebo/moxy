use moxy_token::{Span, Spanner, ToTokenStream, ToTokens, TokenStream};

use crate::*;

/// A chain of casts: `x as u32`, `x as u8 as char`.
///
/// Each cast applies to the expression and the casts before it. Holding the
/// chain in one node keeps every traversal of a long chain from recursing
/// once per cast.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprCastChain {
    pub attrs: Attributes,
    pub expr: Box<Expr>,
    /// One or more `as` and the type cast to.
    pub casts: Vec<(Token![as], Type)>,
}

impl Attributed for ExprCastChain {
    fn attrs(&self) -> &[Attribute] {
        &self.attrs
    }
}

impl From<ExprCastChain> for Expr {
    fn from(value: ExprCastChain) -> Self {
        Self::CastChain(value)
    }
}

impl Spanner for ExprCastChain {
    fn span(&self) -> Span {
        self.to_token_stream().span()
    }
}

impl ToTokens for ExprCastChain {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.expr.to_tokens(t);

        for (as_keyword, ty) in &self.casts {
            as_keyword.to_tokens(t);
            ty.to_tokens(t);
        }
    }
}
