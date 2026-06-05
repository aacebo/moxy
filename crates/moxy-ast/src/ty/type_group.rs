#[allow(unused)]
use crate::*;

#[doc = "A type wrapped in an invisible group delimiter (produced during macro expansion)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TypeGroup {
    pub span: moxy_token::Span,
    pub elem: Box<Type>,
}

impl moxy_token::Spanner for TypeGroup {
    fn span(&self) -> moxy_token::Span {
        self.span
    }
}
