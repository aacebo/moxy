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

impl Spanner for ExprPath {
    fn span(&self) -> Span {
        let start = if let Some(a) = self.attrs.first() {
            a.span()
        } else if let Some(q) = &self.qself {
            q.span()
        } else {
            self.path.span()
        };
        start.join(self.path.span())
    }
}

impl ToTokens for ExprPath {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.path.to_tokens(t);
    }
}

impl ExprPath {
    pub fn into_primary_expr(self) -> super::PrimaryExpr {
        super::PrimaryExpr::from(self)
    }
}
