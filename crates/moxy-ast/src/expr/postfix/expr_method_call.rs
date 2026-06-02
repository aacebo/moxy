use moxy_token::parse::{ParseError, ParseStream};
use moxy_token::token::punct::{Comma, Dot};
use moxy_token::token::{Delim, Group, ToTokens};
use moxy_token::{Span, TokenStream, TokenTree};

use crate::*;

#[doc = "A method call expression: `receiver.method(args)`, `x.collect::<Vec<_>>()`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprMethodCall {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub receiver: Box<super::super::Expr>,
    pub method: Ident,
    pub turbofish: Option<AngleArgs>,
    pub args: Punctuated<super::super::Expr, Comma>,
}

impl ExprMethodCall {
    /// Parse an optional turbofish `::<...>` (method-call generic args).
    pub fn parse_turbofish(stream: &mut ParseStream) -> Result<Option<AngleArgs>, ParseError> {
        let mut fork = stream.fork();

        if fork.peek::<moxy_token::token::punct::PathSep>().is_none() {
            return Ok(None);
        }

        let _ = fork.parse::<moxy_token::token::punct::PathSep>()?;

        if fork.peek::<moxy_token::token::punct::Lt>().is_none() {
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
        Dot::default().to_tokens(t);
        self.method.to_tokens(t);

        if let Some(tf) = &self.turbofish {
            tf.to_tokens(t);
        }

        let mut inner = TokenStream::new();
        self.args.to_tokens(&mut inner);
        t.extend_one(TokenTree::Group(Group::new(Delim::Paren, inner)));
    }
}
