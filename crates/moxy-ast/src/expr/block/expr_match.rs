use moxy_token::keyword::{If, Match};
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::{Comma, FatArrow};
use moxy_token::{Parse, Span, Spanner, ToTokens, TokenStream};

use crate::expr::parse_expr;
use crate::{Attribute, BlockExpr, Delimited, Expr, Pattern};

/// A match expression: `match x { pat => expr, ... }`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprMatch {
    pub attrs: Vec<Attribute>,
    pub match_keyword: Match,
    pub expr: Box<Expr>,
    pub arms: Delimited<Vec<MatchArm>>,
}

impl Spanner for ExprMatch {
    fn span(&self) -> Span {
        let start = if let Some(a) = self.attrs.first() {
            a.span()
        } else {
            self.match_keyword.span()
        };

        start.join(self.arms.span())
    }
}

impl ExprMatch {
    pub fn parse_from(stream: &mut ParseStream) -> Result<Expr, ParseError> {
        let match_keyword = stream.parse::<Match>()?;
        let expr = Box::new(parse_expr(stream, false)?);
        let arms = Delimited::<Vec<MatchArm>>::parse_brace(stream)?;

        Ok(Expr::Block(BlockExpr::Match(Self {
            attrs: Vec::new(),
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
        for a in &self.attrs {
            a.to_tokens(t);
        }

        self.match_keyword.to_tokens(t);
        self.expr.to_tokens(t);
        self.arms.to_tokens(t);
    }
}

/// A single arm of a `match` expression (`pat (if guard)? => body`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct MatchArm {
    pub attrs: Vec<Attribute>,
    pub pat: Pattern,
    pub if_keyword: Option<If>,
    pub guard: Option<Box<Expr>>,
    pub fat_arrow: FatArrow,
    pub body: Expr,
    pub comma: Option<Comma>,
}

impl Spanner for MatchArm {
    fn span(&self) -> Span {
        let start = if let Some(a) = self.attrs.first() {
            a.span()
        } else {
            self.pat.span()
        };

        let end = self.comma.as_ref().map(|c| c.span()).unwrap_or_else(|| self.body.span());
        start.join(end)
    }
}

impl Parse for MatchArm {
    fn parse(stream: &mut moxy_token::parser::ParseStream) -> Result<Self, moxy_token::parser::ParseError> {
        let attrs = stream.parse::<Vec<Attribute>>()?;
        let pat = stream.parse::<Pattern>()?;
        let (if_keyword, guard) = if let Some(if_kw) = stream.parse_if::<If>() {
            (Some(if_kw), Some(Box::new(stream.parse::<Expr>()?)))
        } else {
            (None, None)
        };

        let fat_arrow = stream.parse::<FatArrow>()?;
        let body = stream.parse::<Expr>()?;
        let comma = stream.parse_if::<Comma>();

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
        for a in &self.attrs {
            a.to_tokens(t);
        }

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
