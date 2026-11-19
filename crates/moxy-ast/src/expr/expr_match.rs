use moxy_token::{Span, Spanner, ToTokenStream, ToTokens, TokenStream};

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

impl Attributed for ExprMatch {
    fn attrs(&self) -> &[Attribute] {
        &self.attrs
    }
}

impl From<ExprMatch> for Expr {
    fn from(value: ExprMatch) -> Self {
        Self::Match(value)
    }
}

impl Spanner for ExprMatch {
    fn span(&self) -> Span {
        self.to_token_stream().span()
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

impl Attributed for MatchArm {
    fn attrs(&self) -> &[Attribute] {
        &self.attrs
    }
}

impl MatchArm {
    // The attributes, pattern, guard, and arrow of an arm, with a placeholder body
    fn parse_head(parser: &Parser) -> Result<Box<Self>, ParseError> {
        let attrs = <_ as Parse>::parse(parser)?;
        let pat = <_ as Parse>::parse(parser)?;
        let (if_keyword, guard) = if <Token![if]>::peek(parser.cursor()) {
            (Some(<_ as Parse>::parse(parser)?), Some(<_ as Parse>::parse(parser)?))
        } else {
            (None, None)
        };

        Ok(Box::new(Self {
            attrs,
            pat,
            if_keyword,
            guard,
            fat_arrow: <_ as Parse>::parse(parser)?,
            body: Expr::Verbatim(TokenStream::new()),
            comma: None,
        }))
    }
}

impl Parse for MatchArm {
    fn peek(cursor: Cursor<'_>) -> bool {
        Pattern::peek(Attributes::skip(cursor).unwrap_or(cursor))
    }

    // The arm is built before its body parses and boxed, so a nested arm's
    // stack frame holds a pointer instead of the parts of the arm
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let mut arm = Self::parse_head(parser)?;
        arm.body = *super::parse::early(parser)?;
        arm.comma = <_ as Parse>::parse(parser)?;
        Ok(*arm)
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = Attributes::skip(cursor)?;
        cursor = Pattern::skip(cursor)?;

        if <Token![if]>::peek(cursor) {
            cursor = <Token![if]>::skip(cursor)?;
            cursor = Expr::skip(cursor)?;
        }

        cursor = <Token![=>]>::skip(cursor)?;
        cursor = super::skip::early(cursor)?;
        Option::<Token![,]>::skip(cursor)
    }
}

impl Spanner for MatchArm {
    fn span(&self) -> Span {
        self.to_token_stream().span()
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
