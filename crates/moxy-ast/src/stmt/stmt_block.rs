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
        cursor.is_delimited(moxy_token::Delim::Brace)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            stmts: Delimited::<Vec<Stmt>>::parse_brace(parser)?,
        })
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        let mut inner = cursor.descend(moxy_token::Delim::Brace)?;

        while !inner.is_empty() {
            inner = inner.skip::<Stmt>()?;
        }

        Some(cursor.offset(1))
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
