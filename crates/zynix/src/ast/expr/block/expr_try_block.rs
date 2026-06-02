use crate::ast::*;
use crate::token::ToTokens;
use crate::token::keyword::Try;
use crate::{Span, TokenStream};

#[doc = "A try block expression: `try { ... }`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprTryBlock {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub block: StmtBlock,
}

impl ToTokens for ExprTryBlock {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        Try::default().to_tokens(t);
        self.block.to_tokens(t);
    }
}
