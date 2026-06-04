use moxy_token::parse::{ParseError, ParseStream};
use moxy_token::{Parse, Span, ToTokens, TokenStream};

use super::Stmt;
use crate::Delimited;

#[doc = "A braced block of statements (`{ stmt; stmt; expr }`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct StmtBlock {
    pub span: Span,
    pub brace: Delimited<Vec<Stmt>>,
}

impl Parse for StmtBlock {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let brace = Delimited::<Vec<Stmt>>::parse_brace(stream)?;
        Ok(Self {
            span: Span::default(),
            brace,
        })
    }
}

impl ToTokens for StmtBlock {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.brace.to_tokens(t);
    }
}
