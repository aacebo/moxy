use super::Stmt;
use crate::parse::{ParseError, ParseStream};
use crate::token::{Delim, Group, ToTokens};
use crate::{Parse, Span, TokenStream, TokenTree};

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
        let mut stmts = Vec::new();

        while !inner.is_empty() {
            stmts.push(inner.parse::<Stmt>()?);
        }

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
