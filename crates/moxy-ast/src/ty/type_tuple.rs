use moxy_macros::{Parse, ToTokens};
use moxy_token::Span;
use moxy_token::punct::Comma;

use super::Type;
use crate::{Delimited, Punctuated};

#[doc = "A tuple type (e.g. `()`, `(A, B)`, `(T,)`)."]
#[derive(Debug, Clone, Parse, ToTokens)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TypeTuple {
    #[parse(skip)]
    pub span: Span,
    #[parse(paren, terminated)]
    pub paren: Delimited<Punctuated<Type, Comma>>,
}
