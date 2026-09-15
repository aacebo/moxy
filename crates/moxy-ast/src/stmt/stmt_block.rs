use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use super::Stmt;
use crate::*;

/// A braced block of statements (`{ stmt; stmt; expr }`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct StmtBlock {
    pub stmts: Delimited<Vec<Stmt>>,
}

impl Parse for StmtBlock {
    fn peek(cursor: Cursor<'_>) -> bool {
        !cursor.is_empty()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            stmts: Delimited::<Vec<Stmt>>::parse_brace(parser)?,
        })
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
