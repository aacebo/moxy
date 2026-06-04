use moxy_token::{Span, ToTokens, TokenStream};

use crate::{Attribute, Delimited};

#[doc = "A parenthesized expression: `(x + y)`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprParen {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub paren: Delimited<Box<super::super::Expr>>,
}

impl ToTokens for ExprParen {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.paren.to_tokens(t);
    }
}
