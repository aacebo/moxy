use moxy_token::keyword::Return;
use moxy_token::{Span, ToTokens, TokenStream};

use crate::Attribute;

#[doc = "A return expression: `return`, `return expr`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprReturn {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub expr: Option<Box<super::super::Expr>>,
}

impl ToTokens for ExprReturn {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        Return::default().to_tokens(t);

        if let Some(e) = &self.expr {
            e.to_tokens(t);
        }
    }
}
