use moxy_token::keyword::Const;
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A const block expression: `const { ... }`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprConst {
    pub attrs: Attributes,
    pub const_keyword: Const,
    pub block: StmtBlock,
}

impl Spanner for ExprConst {
    fn span(&self) -> Span {
        let start = if let Some(a) = self.attrs.first() {
            a.span()
        } else {
            self.const_keyword.span()
        };

        start.join(self.block.span())
    }
}

impl ExprConst {
    pub fn parse_from(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let const_keyword = stream.parse::<Const>()?;
        let block = stream.parse::<StmtBlock>()?;

        Ok(Self {
            attrs: Attributes::default(),
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
        for a in &self.attrs {
            a.to_tokens(t);
        }

        self.const_keyword.to_tokens(t);
        self.block.to_tokens(t);
    }
}
