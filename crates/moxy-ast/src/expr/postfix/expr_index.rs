use moxy_token::{Span, ToTokens, TokenStream};

use crate::{Attribute, Delimited};

#[doc = "An index expression: `a[0]`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprIndex {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub base: Box<super::super::Expr>,
    pub bracket: Delimited<Box<super::super::Expr>>,
}

impl ToTokens for ExprIndex {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.base.to_tokens(t);
        self.bracket.to_tokens(t);
    }
}
