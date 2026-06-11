use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A path pattern, e.g. `Some` or `std::option::Option::None`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatPath {
    pub attrs: Attributes,
    pub qself: Option<QSelf>,
    pub path: Path,
}

impl Spanner for PatPath {
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

impl ToTokens for PatPath {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.path.to_tokens(t);
    }
}

impl PatPath {
    pub fn into_pattern(self) -> super::Pattern {
        super::Pattern::from(self)
    }
}
