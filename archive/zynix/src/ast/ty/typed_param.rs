use zynix_macros::{Parse, ToTokens};

use super::Type;
use crate::Span;
use crate::ast::{Attribute, Pattern};
use crate::token::punct::Colon;

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
