use moxy_macros::{Parse, ToTokens};
use moxy_token::Span;
use moxy_token::token::punct::And;

use super::Type;
use crate::{Lifetime, Mutability};

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
