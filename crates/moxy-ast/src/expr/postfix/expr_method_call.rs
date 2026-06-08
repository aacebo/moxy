use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::{Comma, Dot};
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A method call expression: `receiver.method(args)`, `x.collect::<Vec<_>>()`.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprMethodCall {
    pub attrs: Vec<Attribute>,
    pub receiver: Box<Expr>,
    pub dot: Dot,
    pub method: Ident,
    pub turbofish: Option<AngleArgs>,
    pub args: Delimited<Punctuated<Expr, Comma>>,
}

impl Spanner for ExprMethodCall {
    fn span(&self) -> Span {
        let start = if let Some(a) = self.attrs.first() {
            a.span()
        } else {
            self.receiver.span()
        };

        start.join(self.args.span())
    }
}

impl ExprMethodCall {
    /// Parse an optional turbofish `::<...>` (method-call generic args).
    pub fn parse_turbofish(stream: &mut ParseStream) -> Result<Option<AngleArgs>, ParseError> {
        let mut fork = stream.fork();

        if !fork.peek::<moxy_token::punct::PathSep>() {
            return Ok(None);
        }

        let _ = fork.parse::<moxy_token::punct::PathSep>()?;

        if !fork.peek::<moxy_token::punct::Lt>() {
            return Ok(None);
        }

        let args = fork.parse::<AngleArgs>()?;
        stream.seek(&fork);
        Ok(Some(args))
    }
}

impl ToTokens for ExprMethodCall {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.receiver.to_tokens(t);
        self.dot.to_tokens(t);
        self.method.to_tokens(t);

        if let Some(tf) = &self.turbofish {
            tf.to_tokens(t);
        }

        self.args.to_tokens(t);
    }
}
