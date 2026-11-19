use moxy_token::{Span, Spanner, ToTokenStream, ToTokens, TokenStream};

use crate::*;

/// A try expression: `expr?`.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprTry {
    pub attrs: Attributes,
    pub expr: Box<Expr>,
    pub question_punct: Token![?],
}

impl Attributed for ExprTry {
    fn attrs(&self) -> &[Attribute] {
        &self.attrs
    }
}

impl From<ExprTry> for Expr {
    fn from(value: ExprTry) -> Self {
        Self::Try(value)
    }
}

impl Spanner for ExprTry {
    fn span(&self) -> Span {
        self.to_token_stream().span()
    }
}

impl ToTokens for ExprTry {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.expr.to_tokens(t);
        self.question_punct.to_tokens(t);
    }
}
