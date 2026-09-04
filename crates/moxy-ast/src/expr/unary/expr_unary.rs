use crate::Parser;
use crate::Token;
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
        self.attrs.span().join(self.expr.span())
    }
}

impl ExprUnary {
    /// Returns `true` if the parser starts with a prefix unary operator (`!`, `-`, `*`).
    pub fn is_prefix(parser: &Parser) -> bool {
        parser.peek::<Token![!]>() || parser.peek::<Token![-]>() || parser.peek::<Token![*]>()
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
