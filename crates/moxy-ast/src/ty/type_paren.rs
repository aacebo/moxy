use moxy_macros::{Parse, ToTokens};
use moxy_token::Span;

use super::Type;

#[doc = "A parenthesized type (e.g. `(T)`)."]
#[derive(Debug, Clone, Parse, ToTokens)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TypeParen {
    #[parse(skip)]
    pub span: Span,
    #[parse(paren)]
    pub elem: Box<Type>,
}
