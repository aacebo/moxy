use crate::{ParseError, Parser};
use moxy_token::Token;
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A loop expression: `loop { ... }`, `'label: loop { ... }`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprLoop {
    pub attrs: Attributes,
    pub label: Option<Label>,
    pub loop_keyword: Token![loop],
    pub body: StmtBlock,
}

impl Spanner for ExprLoop {
    fn span(&self) -> Span {
        self.attrs.span().join(self.body.span())
    }
}

impl ExprLoop {
    pub fn parse_from(parser: &Parser, label: Option<Label>, attrs: Attributes) -> Result<Self, ParseError> {
        let loop_keyword = parser.parse::<Token![loop]>()?;
        let body = parser.parse::<StmtBlock>()?;

        Ok(Self {
            attrs,
            label,
            loop_keyword,
            body,
        })
    }

    pub fn into_block_expr(self) -> super::BlockExpr {
        super::BlockExpr::from(self)
    }
}

impl ToTokens for ExprLoop {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);

        if let Some(l) = &self.label {
            l.to_tokens(t);
        }

        self.loop_keyword.to_tokens(t);
        self.body.to_tokens(t);
    }
}
