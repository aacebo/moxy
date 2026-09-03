use crate::{ParseError, Parser};
use moxy_token::Token;
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::expr::parse_expr;
use crate::*;

/// A for loop expression: `for pat in expr { ... }`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprForLoop {
    pub attrs: Attributes,
    pub label: Option<Label>,
    pub for_keyword: Token![for],
    pub pat: Box<Pattern>,
    pub in_keyword: Token![in],
    pub expr: Box<Expr>,
    pub body: StmtBlock,
}

impl Spanner for ExprForLoop {
    fn span(&self) -> Span {
        self.attrs.span().join(self.body.span())
    }
}

impl ExprForLoop {
    pub fn parse_from(parser: &Parser, label: Option<Label>, attrs: Attributes) -> Result<Self, ParseError> {
        let for_keyword = parser.parse::<Token![for]>()?;
        let pat = Box::new(parser.parse::<Pattern>()?);
        let in_keyword = parser.parse::<Token![in]>()?;
        let expr = Box::new(parse_expr(parser, false)?);
        let body = parser.parse::<StmtBlock>()?;

        Ok(Self {
            attrs,
            label,
            for_keyword,
            pat,
            in_keyword,
            expr,
            body,
        })
    }

    pub fn into_block_expr(self) -> super::BlockExpr {
        super::BlockExpr::from(self)
    }
}

impl ToTokens for ExprForLoop {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);

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
