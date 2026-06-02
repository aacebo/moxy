use moxy_token::parse::{ParseError, ParseStream};
use moxy_token::{Delim, Group, Parse, Span, ToTokens, TokenStream, TokenTree};

use super::Stmt;

#[doc = "A braced block of statements (`{ stmt; stmt; expr }`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct StmtBlock {
    pub span: Span,
    pub stmts: Vec<Stmt>,
}

impl Parse for StmtBlock {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let group = stream.parse_group(Delim::Brace)?;
        let mut inner = group.parse();
        let stmts = inner.parse_until_empty::<Stmt>()?;

        Ok(Self {
            span: Span::default(),
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

        t.extend_one(TokenTree::Group(Group::new(Delim::Brace, inner)));
    }
}
