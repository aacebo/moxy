use crate::{ParseError, Parser};
use moxy_token::Token;
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A const block expression: `const { ... }`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprConst {
    pub attrs: Attributes,
    pub const_keyword: Token![const],
    pub block: StmtBlock,
}

impl Spanner for ExprConst {
    fn span(&self) -> Span {
        self.attrs.span().join(self.block.span())
    }
}

impl ExprConst {
    pub fn parse_from(parser: &Parser, attrs: Attributes) -> Result<Self, ParseError> {
        let const_keyword = parser.parse::<Token![const]>()?;
        let block = parser.parse::<StmtBlock>()?;

        Ok(Self {
            attrs,
            const_keyword,
            block,
        })
    }

    pub fn into_block_expr(self) -> super::BlockExpr {
        super::BlockExpr::from(self)
    }
}

impl ToTokens for ExprConst {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.const_keyword.to_tokens(t);
        self.block.to_tokens(t);
    }
}
