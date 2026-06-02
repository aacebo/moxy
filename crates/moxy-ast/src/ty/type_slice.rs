use moxy_macros::{Parse, ToTokens};
use moxy_token::Span;

use super::Type;

#[doc = "A slice type (e.g. `[T]`)."]
#[derive(Debug, Clone, Parse, ToTokens)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TypeSlice {
    #[parse(skip)]
    pub span: Span,
    #[parse(bracket)]
    pub elem: Box<Type>,
}
