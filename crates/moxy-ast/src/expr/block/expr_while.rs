use moxy_token::keyword::While;
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::expr::parse_expr;
use crate::*;

/// A while loop expression: `while cond { ... }`, `while let pat = expr { ... }`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprWhile {
    pub attrs: Vec<Attribute>,
    pub label: Option<Label>,
    pub while_keyword: While,
    pub cond: Box<Expr>,
    pub body: StmtBlock,
}

impl Spanner for ExprWhile {
    fn span(&self) -> Span {
        let start = if let Some(a) = self.attrs.first() {
            a.span()
        } else if let Some(l) = &self.label {
            l.span()
        } else {
            self.while_keyword.span()
        };

        start.join(self.body.span())
    }
}

impl ExprWhile {
    pub fn parse_from(stream: &mut ParseStream, label: Option<Label>) -> Result<Self, ParseError> {
        let while_keyword = stream.parse::<While>()?;
        let cond = Box::new(parse_expr(stream, false)?);
        let body = stream.parse::<StmtBlock>()?;

        Ok(Self {
            attrs: Vec::new(),
            label,
            while_keyword,
            cond,
            body,
        })
    }

    pub fn into_block_expr(self) -> super::BlockExpr {
        super::BlockExpr::from(self)
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
