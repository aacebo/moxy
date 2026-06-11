use moxy_token::parser::ParseStream;
use moxy_token::punct::{Not, Star};
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A unary expression: `!x`, `-x`, `*x`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprUnary {
    pub attrs: Attributes,
    pub op: UnOp,
    pub expr: Box<Expr>,
}

impl Spanner for ExprUnary {
    fn span(&self) -> Span {
        let start = if let Some(a) = self.attrs.first() {
            a.span()
        } else {
            self.op.span()
        };
        start.join(self.expr.span())
    }
}

impl ExprUnary {
    /// Returns `true` if the stream starts with a prefix unary operator (`!`, `-`, `*`).
    pub fn is_prefix(stream: &mut ParseStream) -> bool {
        stream.peek::<Not>() || stream.peek::<moxy_token::punct::Minus>() || stream.peek::<Star>()
    }

    pub fn into_unary_expr(self) -> super::UnaryExpr {
        super::UnaryExpr::from(self)
    }
}

impl ToTokens for ExprUnary {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.op.to_tokens(t);
        self.expr.to_tokens(t);
    }
}
