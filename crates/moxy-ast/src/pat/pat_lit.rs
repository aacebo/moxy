use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A literal pattern, e.g. `42`, `'a'`, or `"hello"`.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatLit {
    pub attrs: Vec<Attribute>,
    pub expr: Expr,
}

impl Spanner for PatLit {
    fn span(&self) -> Span {
        let start = if let Some(a) = self.attrs.first() {
            a.span()
        } else {
            self.expr.span()
        };
        start.join(self.expr.span())
    }
}

impl ToTokens for PatLit {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }

        self.expr.to_tokens(t);
    }
}

impl PatLit {
    pub fn into_pattern(self) -> super::Pattern {
        super::Pattern::from(self)
    }
}
