use moxy_token::keyword::While;
use moxy_token::parse::{ParseError, ParseStream};
use moxy_token::{Span, ToTokens, TokenStream};

use crate::*;

#[doc = "A while loop expression: `while cond { ... }`, `while let pat = expr { ... }`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprWhile {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub label: Option<Label>,
    pub while_keyword: While,
    pub cond: Box<super::super::Expr>,
    pub body: StmtBlock,
}

impl ExprWhile {
    pub fn parse_from(stream: &mut ParseStream, label: Option<Label>) -> Result<Self, ParseError> {
        let while_keyword = stream.parse::<While>()?;
        let cond = Box::new(super::super::parse_expr(stream, false)?);
        let body = stream.parse::<StmtBlock>()?;
        Ok(Self {
            span: Span::default(),
            attrs: Vec::new(),
            label,
            while_keyword,
            cond,
            body,
        })
    }
}

impl ToTokens for ExprWhile {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }

        if let Some(l) = &self.label {
            l.to_tokens(t);
        }

        self.while_keyword.to_tokens(t);
        self.cond.to_tokens(t);
        self.body.to_tokens(t);
    }
}
