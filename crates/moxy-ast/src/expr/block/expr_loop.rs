use moxy_token::token::ToTokens;
use moxy_token::token::keyword::Loop;
use moxy_token::{Span, TokenStream};

use crate::*;

#[doc = "A loop expression: `loop { ... }`, `'label: loop { ... }`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprLoop {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub label: Option<Label>,
    pub body: StmtBlock,
}

impl ToTokens for ExprLoop {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }

        if let Some(l) = &self.label {
            l.to_tokens(t);
        }

        Loop::default().to_tokens(t);
        self.body.to_tokens(t);
    }
}
