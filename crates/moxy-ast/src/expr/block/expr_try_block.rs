use moxy_token::keyword::Try;
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::{Span, ToTokens, TokenStream};

use crate::*;

#[doc = "A try block expression: `try { ... }`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprTryBlock {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub try_keyword: Try,
    pub block: StmtBlock,
}

impl ExprTryBlock {
    pub fn parse_from(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let try_keyword = stream.parse::<Try>()?;
        let block = stream.parse::<StmtBlock>()?;
        Ok(Self {
            span: Span::default(),
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
