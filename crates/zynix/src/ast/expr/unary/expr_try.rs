use crate::ast::Attribute;
use crate::token::ToTokens;
use crate::token::punct::Question;
use crate::{Span, TokenStream};

#[doc = "A try expression: `expr?`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprTry {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub expr: Box<super::super::Expr>,
}

impl ToTokens for ExprTry {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.expr.to_tokens(t);
        Question::default().to_tokens(t);
    }
}
