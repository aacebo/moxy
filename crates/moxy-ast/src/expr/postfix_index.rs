use moxy_token::{Span, Spanner, ToTokenStream, ToTokens, TokenStream};

use crate::*;

/// An index into the expression before it: `[0]` in `a[0]`.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PostfixIndex {
    pub index: Delimited<Box<Expr>>,
}

impl Spanner for PostfixIndex {
    fn span(&self) -> Span {
        self.to_token_stream().span()
    }
}

impl ToTokens for PostfixIndex {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.index.to_tokens(t);
    }
}

impl From<PostfixIndex> for Postfix {
    fn from(value: PostfixIndex) -> Self {
        Self::Index(value)
    }
}
