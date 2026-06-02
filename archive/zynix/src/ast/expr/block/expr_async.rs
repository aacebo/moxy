use super::ExprBrace;
use crate::ast::*;
use crate::parse::ParseStream;
use crate::token::ToTokens;
use crate::token::keyword::{Async, Move};
use crate::{Span, TokenStream};

#[doc = "An async block expression: `async { ... }`, `async move { ... }`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprAsync {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub capture: bool,
    pub block: StmtBlock,
}

impl ExprAsync {
    /// Returns `true` when the current async keyword is followed by a block
    /// (`async { }` or `async move { }`), as opposed to an async closure.
    pub fn is_block(stream: &ParseStream) -> bool {
        if ExprBrace::is_next(stream) {
            return true;
        }

        matches!(stream.nth(1), Some(tt) if tt.name().as_deref() == Some("move"))
            && matches!(stream.nth(2), Some(crate::token::TokenTree::Group(g)) if g.delim() == crate::token::Delim::Brace)
    }
}

impl ToTokens for ExprAsync {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        Async::default().to_tokens(t);

        if self.capture {
            Move::default().to_tokens(t);
        }

        self.block.to_tokens(t);
    }
}
