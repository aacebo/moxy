use moxy_token::token::ToTokens;
use moxy_token::token::keyword::Break;
use moxy_token::{Span, TokenStream};

use crate::*;

#[doc = "A break expression: `break`, `break 'label`, `break expr`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprBreak {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub label: Option<Label>,
    pub expr: Option<Box<super::super::Expr>>,
}

impl ToTokens for ExprBreak {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        Break::default().to_tokens(t);

        if let Some(l) = &self.label {
            l.name.to_tokens(t);
        }

        if let Some(e) = &self.expr {
            e.to_tokens(t);
        }
    }
}
