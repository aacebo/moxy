use moxy_token::keyword::Try;
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A try block expression: `try { ... }`.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprTryBlock {
    pub attrs: Vec<Attribute>,
    pub try_keyword: Try,
    pub block: StmtBlock,
}

impl Spanner for ExprTryBlock {
    fn span(&self) -> Span {
        let start = if let Some(a) = self.attrs.first() {
            a.span()
        } else {
            self.try_keyword.span()
        };

        start.join(self.block.span())
    }
}

impl ExprTryBlock {
    pub fn parse_from(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let try_keyword = stream.parse::<Try>()?;
        let block = stream.parse::<StmtBlock>()?;

        Ok(Self {
            attrs: Vec::new(),
            try_keyword,
            block,
        })
    }
}

impl ToTokens for ExprTryBlock {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }

        self.try_keyword.to_tokens(t);
        self.block.to_tokens(t);
    }
}
