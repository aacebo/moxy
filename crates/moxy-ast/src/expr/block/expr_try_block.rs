use crate::{ParseError, Parser};
use moxy_token::Token;
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A try block expression: `try { ... }`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprTryBlock {
    pub attrs: Attributes,
    pub try_keyword: Token![try],
    pub block: StmtBlock,
}

impl Spanner for ExprTryBlock {
    fn span(&self) -> Span {
        self.attrs.span().join(self.block.span())
    }
}

impl ExprTryBlock {
    pub fn parse_from(parser: &Parser, attrs: Attributes) -> Result<Self, ParseError> {
        let try_keyword = parser.parse::<Token![try]>()?;
        let block = parser.parse::<StmtBlock>()?;

        Ok(Self {
            attrs,
            try_keyword,
            block,
        })
    }

    pub fn into_block_expr(self) -> super::BlockExpr {
        super::BlockExpr::from(self)
    }
}

impl ToTokens for ExprTryBlock {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.try_keyword.to_tokens(t);
        self.block.to_tokens(t);
    }
}
