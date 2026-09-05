use crate::Token;
use crate::{ParseError, Parser};
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A range expression: `0..10`, `a..=b`, `..`, `a..`, `..b`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprRange {
    pub attrs: Attributes,
    pub start: Option<Box<Expr>>,
    pub limits: RangeLimits,
    pub end: Option<Box<Expr>>,
}

impl Spanner for ExprRange {
    fn span(&self) -> Span {
        let end = if let Some(e) = &self.end {
            e.span()
        } else {
            self.limits.span()
        };

        self.attrs.span().join(end)
    }
}

impl ExprRange {
    /// Parse an optional range end — `None` if the next token cannot begin an expression.
    pub fn maybe_end(parser: &Parser, allow_struct: bool) -> Result<Option<Box<Expr>>, ParseError> {
        use crate::precedence::Precedence;

        if parser.is_empty() || parser.peek::<Token![;]>() || parser.peek::<Token![,]>() {
            return Ok(None);
        }

        let lookahead = parser.lookahead();

        if expr::unary::UnaryExpr::parse_from(&lookahead, allow_struct).is_err() {
            return Ok(None);
        }

        let e = expr::unary::UnaryExpr::parse_from(parser, allow_struct)?;
        let e = super::BinaryExpr::parse_from(parser, e, Precedence::Range.next(), allow_struct)?;
        Ok(Some(Box::new(e)))
    }

    pub fn into_binary_expr(self) -> super::BinaryExpr {
        super::BinaryExpr::from(self)
    }
}

impl ToTokens for ExprRange {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);

        if let Some(s) = &self.start {
            s.to_tokens(t);
        }

        self.limits.to_tokens(t);

        if let Some(e) = &self.end {
            e.to_tokens(t);
        }
    }
}
