use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A range expression: `0..10`, `a..=b`, `..`, `a..`, `..b`.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprRange {
    pub attrs: Vec<Attribute>,
    pub start: Option<Box<Expr>>,
    pub limits: RangeLimits,
    pub end: Option<Box<Expr>>,
}

impl Spanner for ExprRange {
    fn span(&self) -> Span {
        let start = if let Some(a) = self.attrs.first() {
            a.span()
        } else if let Some(s) = &self.start {
            s.span()
        } else {
            self.limits.span()
        };

        let end = if let Some(e) = &self.end {
            e.span()
        } else {
            self.limits.span()
        };

        start.join(end)
    }
}

impl ExprRange {
    /// Parse an optional range end — `None` if the next token cannot begin an expression.
    pub fn maybe_end(stream: &mut ParseStream, allow_struct: bool) -> Result<Option<Box<Expr>>, ParseError> {
        use moxy_token::punct::{Comma, Semi};

        if stream.is_empty() || stream.peek::<Semi>() || stream.peek::<Comma>() {
            return Ok(None);
        }

        let mut fork = stream.fork();

        match expr::unary::UnaryExpr::parse_from(&mut fork, allow_struct) {
            Err(_) => Ok(None),
            Ok(e) => {
                use crate::precedence::Precedence;
                let e = super::BinaryExpr::parse_from(&mut fork, e, Precedence::Range.next(), allow_struct)?;
                stream.seek(&fork);
                Ok(Some(Box::new(e)))
            }
        }
    }
}

impl ToTokens for ExprRange {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }

        if let Some(s) = &self.start {
            s.to_tokens(t);
        }

        self.limits.to_tokens(t);

        if let Some(e) = &self.end {
            e.to_tokens(t);
        }
    }
}
