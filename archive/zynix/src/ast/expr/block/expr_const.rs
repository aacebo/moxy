use crate::ast::*;
use crate::token::ToTokens;
use crate::token::keyword::Const;
use crate::{Span, TokenStream};

#[doc = "A const block expression: `const { ... }`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprConst {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub block: StmtBlock,
}

impl ToTokens for ExprConst {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        Const::default().to_tokens(t);
        self.block.to_tokens(t);
    }
}
