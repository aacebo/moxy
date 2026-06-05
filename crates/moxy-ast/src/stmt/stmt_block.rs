use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::{Parse, Span, Spanner, ToTokens, TokenStream};

use super::Stmt;
use crate::Delimited;

#[doc = "A braced block of statements (`{ stmt; stmt; expr }`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct StmtBlock {
    pub stmts: Delimited<Vec<Stmt>>,
}

impl Parse for StmtBlock {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let stmts = Delimited::<Vec<Stmt>>::parse_brace(stream)?;
        Ok(Self { stmts })
    }
}

impl Spanner for StmtBlock {
    fn span(&self) -> Span {
        self.stmts.span()
    }
}

impl ToTokens for StmtBlock {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.stmts.to_tokens(t);
    }
}
