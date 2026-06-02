use moxy_token::keyword::Unsafe;
use moxy_token::parse::{ParseError, ParseStream};
use moxy_token::{Span, ToTokens, TokenStream};

use crate::*;

#[doc = "An unsafe block expression: `unsafe { ... }`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprUnsafe {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub unsafe_keyword: Unsafe,
    pub block: StmtBlock,
}

impl ExprUnsafe {
    pub fn parse_from(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let unsafe_keyword = stream.parse::<Unsafe>()?;
        let block = stream.parse::<StmtBlock>()?;
        Ok(Self {
            span: Span::default(),
            attrs: Vec::new(),
            unsafe_keyword,
            block,
        })
    }
}

impl ToTokens for ExprUnsafe {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.unsafe_keyword.to_tokens(t);
        self.block.to_tokens(t);
    }
}
