use crate::{ParseError, Parser};
use moxy_token::Token;
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use super::ExprBrace;
use crate::*;

/// An async block expression: `async { ... }`, `async move { ... }`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprAsync {
    pub attrs: Attributes,
    pub async_keyword: Token![async],
    pub move_keyword: Option<Token![move]>,
    pub block: StmtBlock,
}

impl Spanner for ExprAsync {
    fn span(&self) -> Span {
        self.attrs.span().join(self.block.span())
    }
}

impl ExprAsync {
    pub fn is_block(parser: &Parser) -> bool {
        if ExprBrace::is_next(parser) {
            return true;
        }

        matches!(parser.nth(1), Some(tt) if tt.text() == Some("move"))
            && matches!(parser.nth(2), Some(moxy_token::TokenTree::Group(g)) if g.delim() == moxy_token::Delim::Brace)
    }

    pub fn parse_from(parser: &Parser, attrs: Attributes) -> Result<Self, ParseError> {
        let async_keyword = parser.parse::<Token![async]>()?;
        let move_keyword = parser.parse_if::<Token![move]>();
        let block = parser.parse::<StmtBlock>()?;

        Ok(Self {
            attrs,
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
