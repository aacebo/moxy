use moxy_token::keyword::{If, Match};
use moxy_token::parse::{ParseError, ParseStream};
use moxy_token::punct::{Comma, FatArrow};
use moxy_token::{Brace, Parse, Span, ToTokens, TokenStream};

use crate::{Attribute, Expr, Pattern};

#[doc = "A match expression: `match x { pat => expr, ... }`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprMatch {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub match_keyword: Match,
    pub expr: Box<Expr>,
    pub brace: Brace,
    pub arms: Vec<MatchArm>,
}

impl ExprMatch {
    pub fn parse_from(stream: &mut ParseStream) -> Result<Expr, ParseError> {
        let match_keyword = stream.parse::<Match>()?;
        let expr = Box::new(super::super::parse_expr(stream, false)?);
        let (brace, group) = stream.parse_brace()?;
        let mut inner = group.parse();
        let arms = inner.parse::<Vec<MatchArm>>()?;
        Ok(Expr::Block(super::BlockExpr::Match(Self {
            span: Span::default(),
            attrs: Vec::new(),
            match_keyword,
            expr,
            brace,
            arms,
        })))
    }
}

impl ToTokens for ExprMatch {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.match_keyword.to_tokens(t);
        self.expr.to_tokens(t);
        let mut inner = TokenStream::new();
        for arm in &self.arms {
            arm.to_tokens(&mut inner);
        }
        self.brace.surround(t, inner);
    }
}

#[doc = "A single arm of a `match` expression (`pat (if guard)? => body`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct MatchArm {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub pat: Pattern,
    pub if_keyword: Option<If>,
    pub guard: Option<Box<Expr>>,
    pub fat_arrow: FatArrow,
    pub body: Expr,
    pub comma: Option<Comma>,
}

impl Parse for MatchArm {
    fn parse(stream: &mut moxy_token::parse::ParseStream) -> Result<Self, moxy_token::parse::ParseError> {
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
            span: Span::default(),
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
