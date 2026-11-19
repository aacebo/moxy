use moxy_token::{Span, Spanner, ToTokenStream, ToTokens, TokenStream};

use crate::*;

/// A try of the expression before it: `?` in `expr?`.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PostfixTry {
    pub question_punct: Token![?],
}

impl Spanner for PostfixTry {
    fn span(&self) -> Span {
        self.to_token_stream().span()
    }
}

impl ToTokens for PostfixTry {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.question_punct.to_tokens(t);
    }
}

impl From<PostfixTry> for Postfix {
    fn from(value: PostfixTry) -> Self {
        Self::Try(value)
    }
}
