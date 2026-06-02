use moxy_token::keyword::{For, In};
use moxy_token::parse::{ParseError, ParseStream};
use moxy_token::{Span, ToTokens, TokenStream};

use crate::*;

#[doc = "A for loop expression: `for pat in expr { ... }`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprForLoop {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub label: Option<Label>,
    pub for_keyword: For,
    pub pat: Box<Pattern>,
    pub in_keyword: In,
    pub expr: Box<super::super::Expr>,
    pub body: StmtBlock,
}

impl ExprForLoop {
    pub fn parse_from(stream: &mut ParseStream, label: Option<Label>) -> Result<Self, ParseError> {
        let for_keyword = stream.parse::<For>()?;
        let pat = Box::new(stream.parse::<Pattern>()?);
        let in_keyword = stream.parse::<In>()?;
        let expr = Box::new(super::super::parse_expr(stream, false)?);
        let body = stream.parse::<StmtBlock>()?;
        Ok(Self {
            span: Span::default(),
            attrs: Vec::new(),
            label,
            for_keyword,
            pat,
            in_keyword,
            expr,
            body,
        })
    }
}

impl ToTokens for ExprForLoop {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }

        if let Some(l) = &self.label {
            l.to_tokens(t);
        }

        self.for_keyword.to_tokens(t);
        self.pat.to_tokens(t);
        self.in_keyword.to_tokens(t);
        self.expr.to_tokens(t);
        self.body.to_tokens(t);
    }
}
