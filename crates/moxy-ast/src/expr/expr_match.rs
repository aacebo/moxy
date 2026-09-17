use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A match expression: `match x { pat => expr, ... }`.
#[derive(Debug, Clone, PartialEq, Eq)]
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

impl Parse for MatchArm {
    fn peek(cursor: Cursor<'_>) -> bool {
        Attributes::skip(cursor).unwrap_or(cursor).peek::<Pattern>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = parser.parse()?;
        let pat = parser.parse()?;
        let (if_keyword, guard) = if parser.peek::<Token![if]>() {
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
        cursor = cursor.skip::<Pattern>()?;

        if cursor.peek::<Token![if]>() {
            cursor = cursor.skip::<Token![if]>()?;
            cursor = cursor.skip::<Expr>()?;
        }

        cursor = cursor.skip::<Token![=>]>()?;
        cursor = cursor.skip::<Expr>()?;
        cursor.skip::<Option<Token![,]>>()
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
