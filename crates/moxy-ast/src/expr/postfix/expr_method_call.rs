use crate::Token;
use crate::{ParseError, Parser};
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A method call expression: `receiver.method(args)`, `x.collect::<Vec<_>>()`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprMethodCall {
    pub attrs: Attributes,
    pub receiver: Box<Expr>,
    pub dot: Token![.],
    pub method: Ident,
    pub turbofish: Option<AngleArguments>,
    pub args: Delimited<Punctuated<Expr, Token![,]>>,
}

impl Spanner for ExprMethodCall {
    fn span(&self) -> Span {
        self.attrs.span().join(self.args.span())
    }
}

impl ExprMethodCall {
    /// Parse an optional turbofish `::<...>` (method-call generic args).
    pub fn parse_turbofish(parser: &Parser) -> Result<Option<AngleArguments>, ParseError> {
        Ok(parser.parse_if())
    }

    pub fn into_postfix_expr(self) -> super::PostfixExpr {
        super::PostfixExpr::from(self)
    }
}

impl ToTokens for ExprMethodCall {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.receiver.to_tokens(t);
        self.dot.to_tokens(t);
        self.method.to_tokens(t);

        if let Some(tf) = &self.turbofish {
            tf.to_tokens(t);
        }

        self.args.to_tokens(t);
    }
}
