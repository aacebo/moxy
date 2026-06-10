use moxy_token::{Eq, Span, Spanner, ToTokens, TokenStream};

use crate::{Expr, Meta, Path};

/// A name-value meta item (`name = expr`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct MetaNameValue {
    pub path: Path,
    pub eq: Eq,
    pub value: Expr,
}

impl MetaNameValue {
    pub fn into_meta(self) -> Meta {
        Meta::NameValue(self)
    }
}

impl Spanner for MetaNameValue {
    fn span(&self) -> Span {
        self.path.span().join(self.value.span())
    }
}

impl ToTokens for MetaNameValue {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.path.to_tokens(t);
        self.eq.to_tokens(t);
        self.value.to_tokens(t);
    }
}
