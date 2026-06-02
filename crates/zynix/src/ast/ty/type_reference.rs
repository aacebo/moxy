use zynix_macros::{Parse, ToTokens};

use super::Type;
use crate::Span;
use crate::ast::{Lifetime, Mutability};
use crate::token::punct::And;

#[doc = "A reference type (e.g. `&'a T`, `&mut T`)."]
#[derive(Debug, Clone, Parse, ToTokens)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TypeReference {
    #[parse(skip)]
    pub span: Span,
    #[parse(prefix = And)]
    pub lifetime: Option<Lifetime>,
    pub mutability: Mutability,
    pub elem: Box<Type>,
}
