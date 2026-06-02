use moxy_macros::{Parse, ToTokens};
use moxy_token::Span;
use moxy_token::punct::Colon;

use super::Type;
use crate::{Attribute, Pattern};

#[doc = "A typed function parameter (`pat: Type`)."]
#[derive(Debug, Clone, Parse, ToTokens)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TypedParam {
    #[parse(skip)]
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub pat: Box<Pattern>,
    #[parse(prefix = Colon)]
    pub ty: Box<Type>,
}
