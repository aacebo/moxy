use moxy_token::{Span, Spanner, ToTokenStream, ToTokens, TokenStream};

use crate::*;

/// An await of the expression before it: `.await` in `expr.await`.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PostfixAwait {
    pub dot: Token![.],
    pub await_keyword: Token![await],
}

impl Spanner for PostfixAwait {
    fn span(&self) -> Span {
        self.to_token_stream().span()
    }
}

impl ToTokens for PostfixAwait {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.dot.to_tokens(t);
        self.await_keyword.to_tokens(t);
    }
}

impl From<PostfixAwait> for Postfix {
    fn from(value: PostfixAwait) -> Self {
        Self::Await(value)
    }
}
