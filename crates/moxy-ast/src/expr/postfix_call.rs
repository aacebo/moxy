use moxy_token::{Span, Spanner, ToTokenStream, ToTokens, TokenStream};

use crate::*;

/// A call of the expression before it: `(a, b)` in `f(a, b)`.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PostfixCall {
    pub args: Delimited<List<Expr, Token![,]>>,
}

impl Spanner for PostfixCall {
    fn span(&self) -> Span {
        self.to_token_stream().span()
    }
}

impl ToTokens for PostfixCall {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.args.to_tokens(t);
    }
}

impl From<PostfixCall> for Postfix {
    fn from(value: PostfixCall) -> Self {
        Self::Call(value)
    }
}
