use moxy_token::keyword::Const;
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::{Span, ToTokens, TokenStream};

use crate::*;

#[doc = "A const block expression: `const { ... }`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprConst {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub const_keyword: Const,
    pub block: StmtBlock,
}

impl ExprConst {
    pub fn parse_from(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let const_keyword = stream.parse::<Const>()?;
        let block = stream.parse::<StmtBlock>()?;
        Ok(Self {
            span: Span::default(),
            attrs: Vec::new(),
            const_keyword,
            block,
        })
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
