use moxy_macros::{Parse, ToTokens};
use moxy_token::{Bracket, Span};

use super::Type;

#[doc = "A slice type (e.g. `[T]`)."]
#[derive(Debug, Clone, Parse, ToTokens)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TypeSlice {
    #[parse(skip)]
    pub span: Span,
    pub bracket: Bracket,
    #[parse(bracket = bracket)]
    pub elem: Box<Type>,
}
