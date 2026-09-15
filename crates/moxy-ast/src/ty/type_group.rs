use moxy_token::{Span, Spanner, ToTokens};

use crate::*;

/// A type wrapped in an invisible group delimiter (produced during macro expansion).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TypeGroup {
    pub span: Span,
    pub elem: Box<Type>,
}

impl Spanner for TypeGroup {
    fn span(&self) -> Span {
        self.span
    }
}

impl ToTokens for TypeGroup {
    fn to_tokens(&self, tokens: &mut moxy_token::TokenStream) {
        self.elem.to_tokens(tokens);
    }
}
