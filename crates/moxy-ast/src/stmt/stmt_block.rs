use moxy_token::parse::{ParseError, ParseStream};
use moxy_token::{Brace, Parse, Span, ToTokens, TokenStream};

use super::Stmt;

#[doc = "A braced block of statements (`{ stmt; stmt; expr }`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct StmtBlock {
    pub span: Span,
    pub brace: Brace,
    pub stmts: Vec<Stmt>,
}

impl Parse for StmtBlock {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let (brace, group) = stream.parse_brace()?;
        let mut inner = group.parse();
        let stmts = inner.parse_until_empty::<Stmt>()?;

        Ok(Self {
            span: Span::default(),
            brace,
            stmts,
        })
    }
}

impl ToTokens for StmtBlock {
    fn to_tokens(&self, t: &mut TokenStream) {
        let mut inner = TokenStream::new();

        for s in &self.stmts {
            s.to_tokens(&mut inner);
        }

        self.brace.surround(t, inner);
    }
}
