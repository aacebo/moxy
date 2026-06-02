use crate::ast::*;
use crate::token::ToTokens;
use crate::token::keyword::Unsafe;
use crate::{Span, TokenStream};

#[doc = "An unsafe block expression: `unsafe { ... }`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprUnsafe {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub block: StmtBlock,
}

impl ToTokens for ExprUnsafe {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        Unsafe::default().to_tokens(t);
        self.block.to_tokens(t);
    }
}
