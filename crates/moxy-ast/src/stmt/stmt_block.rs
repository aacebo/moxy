use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use super::Stmt;
use crate::*;

/// A braced block of statements (`{ stmt; stmt; expr }`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct StmtBlock {
    /// The inner attributes that open the body, as `#![a]`.
    pub inner_attrs: Attributes,
    pub stmts: Delimited<Vec<Stmt>>,
}

impl Parse for StmtBlock {
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.is_delimited(moxy_token::Delim::Brace)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        // A block nests its statements one level
        let _depth = moxy_token::__private::Depth::enter().ok_or_else(|| ParseError::too_deep(parser.span()))?;
        let mut inner_attrs = Attributes::default();
        let stmts = Delimited::parse_brace_with(parser, |parser| {
            inner_attrs = Attributes::parse_inner(parser)?;
            <Vec<Stmt> as Parse>::parse(parser)
        })?;

        Ok(Self { inner_attrs, stmts })
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        let _depth = moxy_token::__private::Depth::enter()?;
        let mut inner = Attributes::skip_inner(cursor.descend(moxy_token::Delim::Brace)?)?;

        while !inner.is_empty() {
            inner = Stmt::skip(inner)?;
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
        let mut body = TokenStream::new();
        self.inner_attrs.to_tokens(&mut body);
        self.stmts.inner.to_tokens(&mut body);
        self.stmts.surround(t, body);
    }
}
