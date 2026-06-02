use moxy_token::token::ToTokens;
use moxy_token::token::keyword::Await as KwAwait;
use moxy_token::token::punct::Dot;
use moxy_token::{Span, TokenStream};

use crate::Attribute;

#[doc = "An await expression: `expr.await`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprAwait {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub base: Box<super::super::Expr>,
}

impl ToTokens for ExprAwait {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.base.to_tokens(t);
        Dot::default().to_tokens(t);
        KwAwait::default().to_tokens(t);
    }
}
