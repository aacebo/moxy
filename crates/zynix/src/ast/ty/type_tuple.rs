use zynix_macros::{Parse, ToTokens};

use super::Type;
use crate::Span;
use crate::ast::Punctuated;
use crate::token::punct::Comma;

#[doc = "A tuple type (e.g. `()`, `(A, B)`, `(T,)`)."]
#[derive(Debug, Clone, Parse, ToTokens)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TypeTuple {
    #[parse(skip)]
    pub span: Span,
    #[parse(paren, terminated)]
    pub elems: Punctuated<Type, Comma>,
}
