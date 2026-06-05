use moxy_token::keyword::{Else, If};
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

#[doc = "An if expression: `if cond { ... } else { ... }`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprIf {
    pub attrs: Vec<Attribute>,
    pub if_keyword: If,
    pub cond: Box<super::super::Expr>,
    pub then_branch: StmtBlock,
    pub else_keyword: Option<Else>,
    pub else_branch: Option<Box<super::super::Expr>>,
}

impl Spanner for ExprIf {
    fn span(&self) -> Span {
        let start = if let Some(a) = self.attrs.first() {
            a.span()
        } else {
            self.if_keyword.span()
        };
        let end = if let Some(e) = &self.else_branch {
            e.span()
        } else {
            self.then_branch.span()
        };
        start.join(end)
    }
}

impl ExprIf {
    pub fn parse_from(stream: &mut ParseStream) -> Result<super::super::Expr, ParseError> {
        let if_keyword = stream.parse::<If>()?;
        let cond = Box::new(super::super::parse_expr(stream, false)?);
        let then_branch = stream.parse::<StmtBlock>()?;

        let (else_keyword, else_branch) = if matches!(stream.curr(), Some(tt) if tt.name().as_deref() == Some("else")) {
            let else_kw = stream.parse::<Else>()?;
            let branch = Some(Box::new(crate::PrimaryExpr::parse_from(stream, true)?));
            (Some(else_kw), branch)
        } else {
            (None, None)
        };

        Ok(super::super::Expr::Block(super::BlockExpr::If(Self {
            attrs: Vec::new(),
            if_keyword,
            cond,
            then_branch,
            else_keyword,
            else_branch,
        })))
    }
}

impl ToTokens for ExprIf {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.if_keyword.to_tokens(t);
        self.cond.to_tokens(t);
        self.then_branch.to_tokens(t);

        if let Some(e) = &self.else_branch {
            self.else_keyword.to_tokens(t);
            e.to_tokens(t);
        }
    }
}
