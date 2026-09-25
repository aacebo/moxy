use moxy_token::{Ident, Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A method call expression: `receiver.method(args)`, `x.collect::<Vec<_>>()`.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprMethodCall {
    pub attrs: Attributes,
    pub receiver: Box<Expr>,
    pub dot: Token![.],
    pub method: Ident,
    pub turbofish: Option<AngleArguments>,
    pub args: Delimited<Punctuated<Expr, Token![,]>>,
}

impl From<ExprMethodCall> for Expr {
    fn from(value: ExprMethodCall) -> Self {
        Self::MethodCall(value)
    }
}

impl Spanner for ExprMethodCall {
    fn span(&self) -> Span {
        self.attrs.span().join(self.args.span())
    }
}

impl ToTokens for ExprMethodCall {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.receiver.to_tokens(t);
        self.dot.to_tokens(t);
        self.method.to_tokens(t);
        self.turbofish.to_tokens(t);
        self.args.to_tokens(t);
    }
}
