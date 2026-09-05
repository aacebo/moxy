use crate::Token;
use crate::{ParseError, Parser};
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::expr::parse_expr;
use crate::*;

/// An if expression: `if cond { ... } else { ... }`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprIf {
    pub attrs: Attributes,
    pub if_keyword: Token![if],
    pub cond: Box<Expr>,
    pub then_branch: StmtBlock,
    pub else_keyword: Option<Token![else]>,
    pub else_branch: Option<Box<Expr>>,
}

impl Spanner for ExprIf {
    fn span(&self) -> Span {
        let end = if let Some(e) = &self.else_branch {
            e.span()
        } else {
            self.then_branch.span()
        };

        self.attrs.span().join(end)
    }
}

impl ExprIf {
    pub fn parse_from(parser: &Parser, attrs: Attributes) -> Result<Expr, ParseError> {
        let if_keyword = parser.parse::<Token![if]>()?;
        let cond = Box::new(parse_expr(parser, false)?);
        let then_branch = parser.parse::<StmtBlock>()?;
        let (else_keyword, else_branch) = if matches!(parser.curr(), Some(tt) if tt.text() == Some("else")) {
            let else_kw = parser.parse::<Token![else]>()?;
            let else_attrs = parser.parse::<Attributes>()?;
            let branch = Some(Box::new(PrimaryExpr::parse_from(parser, true, else_attrs)?));
            (Some(else_kw), branch)
        } else {
            (None, None)
        };

        Ok(Expr::Block(BlockExpr::If(Self {
            attrs,
            if_keyword,
            cond,
            then_branch,
            else_keyword,
            else_branch,
        })))
    }

    pub fn into_block_expr(self) -> super::BlockExpr {
        super::BlockExpr::from(self)
    }
}

impl ToTokens for ExprIf {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.if_keyword.to_tokens(t);
        self.cond.to_tokens(t);
        self.then_branch.to_tokens(t);

        if let Some(e) = &self.else_branch {
            self.else_keyword.to_tokens(t);
            e.to_tokens(t);
        }
    }
}
