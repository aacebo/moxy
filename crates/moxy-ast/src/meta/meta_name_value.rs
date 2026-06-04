use moxy_token::{Eq, Span, ToTokens, TokenStream};

use crate::{Expr, Path};

#[doc = "A name-value meta item (`name = expr`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct MetaNameValue {
    pub span: Span,
    pub path: Path,
    pub eq: Eq,
    pub value: Expr,
}

impl ToTokens for MetaNameValue {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.path.to_tokens(t);
        self.eq.to_tokens(t);
        self.value.to_tokens(t);
    }
}
