use crate::Token;
use crate::{ParseError, Parser};
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// An unsafe block expression: `unsafe { ... }`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprUnsafe {
    pub attrs: Attributes,
    pub unsafe_keyword: Token![unsafe],
    pub block: StmtBlock,
}

impl Spanner for ExprUnsafe {
    fn span(&self) -> Span {
        self.attrs.span().join(self.block.span())
    }
}

impl ExprUnsafe {
    pub fn parse_from(parser: &Parser, attrs: Attributes) -> Result<Self, ParseError> {
        let unsafe_keyword = parser.parse::<Token![unsafe]>()?;
        let block = parser.parse::<StmtBlock>()?;

        Ok(Self {
            attrs,
            unsafe_keyword,
            block,
        })
    }

    pub fn into_block_expr(self) -> super::BlockExpr {
        super::BlockExpr::from(self)
    }
}

impl ToTokens for ExprUnsafe {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.unsafe_keyword.to_tokens(t);
        self.block.to_tokens(t);
    }
}
