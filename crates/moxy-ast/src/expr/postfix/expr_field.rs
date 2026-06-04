use moxy_token::punct::Dot;
use moxy_token::{Span, ToTokens, TokenStream};

use crate::*;

#[doc = "A field access expression: `x.field`, `tuple.0`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprField {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub base: Box<super::super::Expr>,
    pub dot: Dot,
    pub member: Member,
}

impl ToTokens for ExprField {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.base.to_tokens(t);
        self.dot.to_tokens(t);
        self.member.to_tokens(t);
    }
}
