use moxy_token::{Span, Spanner, ToTokenStream, ToTokens, TokenStream};

use crate::*;

/// A path expression: `std::mem::swap`, `<T as Trait>::assoc`.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprPath {
    pub attrs: Attributes,
    pub qself: Option<QSelf>,
    pub path: Path,
}

impl Attributed for ExprPath {
    fn attrs(&self) -> &[Attribute] {
        &self.attrs
    }
}

impl From<ExprPath> for Expr {
    fn from(value: ExprPath) -> Self {
        Self::Path(value)
    }
}

impl Spanner for ExprPath {
    fn span(&self) -> Span {
        self.to_token_stream().span()
    }
}

impl ToTokens for ExprPath {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        QSelf::path_to_tokens(self.qself.as_ref(), &self.path, t);
    }
}
