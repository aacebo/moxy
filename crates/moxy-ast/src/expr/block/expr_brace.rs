use moxy_token::parser::ParseStream;
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A block expression: `{ stmts }`, `'label: { stmts }`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprBrace {
    pub attrs: Attributes,
    pub label: Option<Label>,
    pub block: StmtBlock,
}

impl Spanner for ExprBrace {
    fn span(&self) -> Span {
        self.attrs.span().join(self.block.span())
    }
}

impl ExprBrace {
    /// Returns `true` when the token at position 1 (peek-ahead) is a brace group.
    pub fn is_next(stream: &ParseStream) -> bool {
        stream
            .nth(1)
            .and_then(|t| t.as_group())
            .map(|g| g.delim().is_brace())
            .unwrap_or(false)
    }

    pub fn into_block_expr(self) -> super::BlockExpr {
        super::BlockExpr::from(self)
    }
}

impl ToTokens for ExprBrace {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);

        if let Some(l) = &self.label {
            l.to_tokens(t);
        }

        self.block.to_tokens(t);
    }
}
