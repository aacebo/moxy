use moxy_token::{Paren, Span, ToTokens, TokenStream};

use crate::Attribute;

#[doc = "A parenthesized expression: `(x + y)`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprParen {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub paren: Paren,
    pub expr: Box<super::super::Expr>,
}

impl ToTokens for ExprParen {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        let mut inner = TokenStream::new();
        self.expr.to_tokens(&mut inner);
        self.paren.surround(t, inner);
    }
}
