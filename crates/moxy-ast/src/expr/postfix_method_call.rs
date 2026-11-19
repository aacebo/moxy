use moxy_token::{Ident, Span, Spanner, ToTokenStream, ToTokens, TokenStream};

use crate::*;

/// A method call on the expression before it: `.method(args)` in
/// `receiver.method(args)`, `.collect::<Vec<_>>()`.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PostfixMethodCall {
    pub dot: Token![.],
    pub method: Ident,
    pub turbofish: Option<AngleArguments>,
    pub args: Delimited<List<Expr, Token![,]>>,
}

impl Spanner for PostfixMethodCall {
    fn span(&self) -> Span {
        self.to_token_stream().span()
    }
}

impl ToTokens for PostfixMethodCall {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.dot.to_tokens(t);
        self.method.to_tokens(t);
        self.turbofish.to_tokens(t);
        self.args.to_tokens(t);
    }
}

impl From<PostfixMethodCall> for Postfix {
    fn from(value: PostfixMethodCall) -> Self {
        Self::MethodCall(value)
    }
}
