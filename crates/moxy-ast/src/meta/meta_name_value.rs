use moxy_token::token::{Eq, ToTokens};
use moxy_token::{Span, TokenStream};

use crate::{Expr, Path};

#[doc = "A name-value meta item (`name = expr`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct MetaNameValue {
    pub span: Span,
    pub path: Path,
    pub value: Expr,
}

impl ToTokens for MetaNameValue {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.path.to_tokens(t);
        Eq::default().to_tokens(t);
        self.value.to_tokens(t);
    }
}
