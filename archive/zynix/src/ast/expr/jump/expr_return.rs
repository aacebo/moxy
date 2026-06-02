use crate::ast::Attribute;
use crate::token::ToTokens;
use crate::token::keyword::Return;
use crate::{Span, TokenStream};

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
