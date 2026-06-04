use moxy_macros::{Parse, ToTokens};
use moxy_token::{Paren, Span};

use super::Type;

#[doc = "A parenthesized type (e.g. `(T)`)."]
#[derive(Debug, Clone, Parse, ToTokens)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TypeParen {
    #[parse(skip)]
    pub span: Span,
    pub paren: Paren,
    #[parse(paren = paren)]
    pub elem: Box<Type>,
}
