#[allow(unused)]
use crate::ast::*;

#[doc = "A type wrapped in an invisible group delimiter (produced during macro expansion)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TypeGroup {
    pub span: crate::Span,
    pub elem: Box<Type>,
}
