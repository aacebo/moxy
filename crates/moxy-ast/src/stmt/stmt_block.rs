use crate::{Parse, ParseError, Parser};
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use super::Stmt;
use crate::Delimited;

/// A braced block of statements (`{ stmt; stmt; expr }`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct StmtBlock {
    pub stmts: Delimited<Vec<Stmt>>,
}

impl Parse for StmtBlock {
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let stmts = Delimited::<Vec<Stmt>>::parse_brace(parser)?;
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
