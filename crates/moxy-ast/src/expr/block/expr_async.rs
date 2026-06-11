use moxy_token::keyword::{Async, Move};
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use super::ExprBrace;
use crate::*;

/// An async block expression: `async { ... }`, `async move { ... }`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprAsync {
    pub attrs: Attributes,
    pub async_keyword: Async,
    pub move_keyword: Option<Move>,
    pub block: StmtBlock,
}

impl Spanner for ExprAsync {
    fn span(&self) -> Span {
        let start = if let Some(a) = self.attrs.first() {
            a.span()
        } else {
            self.async_keyword.span()
        };

        start.join(self.block.span())
    }
}

impl ExprAsync {
    pub fn is_block(stream: &ParseStream) -> bool {
        if ExprBrace::is_next(stream) {
            return true;
        }

        matches!(stream.nth(1), Some(tt) if tt.text() == Some("move"))
            && matches!(stream.nth(2), Some(moxy_token::TokenTree::Group(g)) if g.delim() == moxy_token::Delim::Brace)
    }

    pub fn parse_from(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let async_keyword = stream.parse::<Async>()?;
        let move_keyword = stream.parse_if::<Move>();
        let block = stream.parse::<StmtBlock>()?;

        Ok(Self {
            attrs: Attributes::default(),
            async_keyword,
            move_keyword,
            block,
        })
    }

    pub fn into_block_expr(self) -> super::BlockExpr {
        super::BlockExpr::from(self)
    }
}

impl ToTokens for ExprAsync {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.async_keyword.to_tokens(t);
        self.move_keyword.to_tokens(t);
        self.block.to_tokens(t);
    }
}
