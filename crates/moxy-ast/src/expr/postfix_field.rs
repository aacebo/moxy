use moxy_token::{Span, Spanner, ToTokenStream, ToTokens, TokenStream};

use crate::*;

/// A field access on the expression before it: `.field` in `x.field`, `.0` in
/// `tuple.0`.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PostfixField {
    pub dot: Token![.],
    pub member: Member,
}

impl Spanner for PostfixField {
    fn span(&self) -> Span {
        self.to_token_stream().span()
    }
}

impl ToTokens for PostfixField {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.dot.to_tokens(t);
        self.member.to_tokens(t);
    }
}

impl From<PostfixField> for Postfix {
    fn from(value: PostfixField) -> Self {
        Self::Field(value)
    }
}
