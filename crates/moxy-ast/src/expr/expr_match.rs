use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A match expression: `match x { pat => expr, ... }`.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprMatch {
    pub attrs: Attributes,
    pub match_keyword: Token![match],
    pub expr: Box<Expr>,
    pub arms: Delimited<Vec<MatchArm>>,
}

impl From<ExprMatch> for Expr {
    fn from(value: ExprMatch) -> Self {
        Self::Match(value)
    }
}

impl Spanner for ExprMatch {
    fn span(&self) -> Span {
        self.attrs.span().join(self.arms.span())
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
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
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

impl Parse for MatchArm {
    fn peek(cursor: Cursor<'_>) -> bool {
        Pattern::peek(Attributes::skip(cursor).unwrap_or(cursor))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = parser.parse()?;
        let pat = parser.parse()?;
        let (if_keyword, guard) = if <Token![if]>::peek(parser.cursor()) {
            (Some(parser.parse()?), Some(parser.parse()?))
        } else {
            (None, None)
        };

        let fat_arrow = parser.parse()?;
        let body = parser.parse()?;
        let comma = parser.parse()?;

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

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = Attributes::skip(cursor)?;
        cursor = Pattern::skip(cursor)?;

        if <Token![if]>::peek(cursor) {
            cursor = <Token![if]>::skip(cursor)?;
            cursor = Expr::skip(cursor)?;
        }

        cursor = <Token![=>]>::skip(cursor)?;
        cursor = Expr::skip(cursor)?;
        Option::<Token![,]>::skip(cursor)
    }
}

impl Spanner for MatchArm {
    fn span(&self) -> Span {
        let end = self.comma.as_ref().map(|c| c.span()).unwrap_or_else(|| self.body.span());
        self.attrs.span().join(end)
    }
}

impl ToTokens for MatchArm {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.pat.to_tokens(t);
        self.if_keyword.to_tokens(t);
        self.guard.to_tokens(t);
        self.fat_arrow.to_tokens(t);
        self.body.to_tokens(t);
        self.comma.to_tokens(t);
    }
}
