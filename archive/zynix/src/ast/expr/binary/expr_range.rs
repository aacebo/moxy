use crate::ast::*;
use crate::parse::{ParseError, ParseStream};
use crate::token::ToTokens;
use crate::{Span, TokenStream};

#[doc = "A range expression: `0..10`, `a..=b`, `..`, `a..`, `..b`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprRange {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub start: Option<Box<super::super::Expr>>,
    pub limits: RangeLimits,
    pub end: Option<Box<super::super::Expr>>,
}

impl ExprRange {
    /// Parse an optional range end — `None` if the next token cannot begin an expression.
    pub fn maybe_end(stream: &mut ParseStream, allow_struct: bool) -> Result<Option<Box<super::super::Expr>>, ParseError> {
        use crate::token::punct::{Comma, Semi};
        if stream.is_empty() || stream.peek::<Semi>().is_some() || stream.peek::<Comma>().is_some() {
            return Ok(None);
        }

        let mut fork = stream.fork();

        match super::super::unary::UnaryExpr::parse_from(&mut fork, allow_struct) {
            Ok(e) => {
                use crate::ast::precedence::Precedence;
                let e = super::BinaryExpr::parse_from(&mut fork, e, Precedence::Range.next(), allow_struct)?;
                stream.seek(&fork);
                Ok(Some(Box::new(e)))
            }
            Err(_) => Ok(None),
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
