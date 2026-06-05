use moxy_token::parser::ParseStream;
use moxy_token::{Delim, Span, Spanner, ToTokens, TokenStream, TokenTree};

use crate::*;

#[doc = "A block expression: `{ stmts }`, `'label: { stmts }`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprBrace {
    pub attrs: Vec<Attribute>,
    pub label: Option<Label>,
    pub block: StmtBlock,
}

impl Spanner for ExprBrace {
    fn span(&self) -> Span {
        let start = if let Some(a) = self.attrs.first() {
            a.span()
        } else if let Some(l) = &self.label {
            l.span()
        } else {
            self.block.span()
        };
        start.join(self.block.span())
    }
}

impl ExprBrace {
    /// Returns `true` when the token at position 1 (peek-ahead) is a brace group.
    pub fn is_next(stream: &ParseStream) -> bool {
        matches!(stream.nth(1), Some(TokenTree::Group(g)) if g.delim() == Delim::Brace)
    }
}

impl ToTokens for ExprBrace {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }

        if let Some(l) = &self.label {
            l.to_tokens(t);
        }

        self.block.to_tokens(t);
    }
}
