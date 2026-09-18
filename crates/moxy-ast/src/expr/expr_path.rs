use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A path expression: `std::mem::swap`, `<T as Trait>::assoc`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprPath {
    pub attrs: Attributes,
    pub qself: Option<QSelf>,
    pub path: Path,
}

impl From<ExprPath> for Expr {
    fn from(value: ExprPath) -> Self {
        Self::Path(value)
    }
}

impl Spanner for ExprPath {
    fn span(&self) -> Span {
        self.attrs.span().join(self.path.span())
    }
}

impl ToTokens for ExprPath {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.path.to_tokens(t);
    }
}
