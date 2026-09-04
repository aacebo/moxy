use crate::{Parse, ParseError, Parser};

use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::expr::parse_expr;
use crate::{Attributes, BlockExpr, Delimited, Expr, Pattern, Token};

/// A match expression: `match x { pat => expr, ... }`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprMatch {
    pub attrs: Attributes,
    pub match_keyword: Token![match],
    pub expr: Box<Expr>,
    pub arms: Delimited<Vec<MatchArm>>,
}

impl Spanner for ExprMatch {
    fn span(&self) -> Span {
        self.attrs.span().join(self.arms.span())
    }
}

impl ExprMatch {
    pub fn parse_from(parser: &Parser, attrs: Attributes) -> Result<Expr, ParseError> {
        let match_keyword = parser.parse::<Token![match]>()?;
        let expr = Box::new(parse_expr(parser, false)?);
        let arms = Delimited::<Vec<MatchArm>>::parse_brace(parser)?;

        Ok(Expr::Block(BlockExpr::Match(Self {
            attrs,
            match_keyword,
            expr,
            arms,
        })))
    }

    pub fn into_block_expr(self) -> super::BlockExpr {
        super::BlockExpr::from(self)
    }
}

impl ToTokens for ExprMatch {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.match_keyword.to_tokens(t);
        self.expr.to_tokens(t);
        self.arms.to_tokens(t);
    }
}

/// A single arm of a `match` expression (`pat (if guard)? => body`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct MatchArm {
    pub attrs: Attributes,
    pub pat: Pattern,
    pub if_keyword: Option<Token![if]>,
    pub guard: Option<Box<Expr>>,
    pub fat_arrow: Token![=>],
    pub body: Expr,
    pub comma: Option<Token![,]>,
}

impl Spanner for MatchArm {
    fn span(&self) -> Span {
        let end = self.comma.as_ref().map(|c| c.span()).unwrap_or_else(|| self.body.span());
        self.attrs.span().join(end)
    }
}

impl Parse for MatchArm {
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = parser.parse::<Attributes>()?;
        let pat = parser.parse::<Pattern>()?;
        let (if_keyword, guard) = if let Some(if_kw) = parser.parse_if::<Token![if]>() {
            (Some(if_kw), Some(Box::new(parser.parse::<Expr>()?)))
        } else {
            (None, None)
        };

        let fat_arrow = parser.parse::<Token![=>]>()?;
        let body = parser.parse::<Expr>()?;
        let comma = parser.parse_if::<Token![,]>();

        Ok(Self {
            attrs,
            pat,
            if_keyword,
            guard,
            fat_arrow,
            body,
            comma,
        })
    }
}

impl ToTokens for MatchArm {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.pat.to_tokens(t);

        if let Some(g) = &self.guard {
            self.if_keyword.to_tokens(t);
            g.to_tokens(t);
        }

        self.fat_arrow.to_tokens(t);
        self.body.to_tokens(t);
        self.comma.to_tokens(t);
    }
}
