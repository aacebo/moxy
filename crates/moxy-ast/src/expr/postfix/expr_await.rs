use moxy_token::keyword::Await as KwAwait;
use moxy_token::punct::Dot;
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// An await expression: `expr.await`.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprAwait {
    pub attrs: Vec<Attribute>,
    pub base: Box<Expr>,
    pub dot: Dot,
    pub await_keyword: KwAwait,
}

impl Spanner for ExprAwait {
    fn span(&self) -> Span {
        let start = if let Some(a) = self.attrs.first() {
            a.span()
        } else {
            self.base.span()
        };
        start.join(self.await_keyword.span())
    }
}

impl ToTokens for ExprAwait {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.base.to_tokens(t);
        self.dot.to_tokens(t);
        self.await_keyword.to_tokens(t);
    }
}
