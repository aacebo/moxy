use moxy_token::parser::ParseStream;
use moxy_token::{Span, Spanner, ToTokens, TokenStream, TokenTree};

use crate::*;

/// A literal expression: `1`, `"hello"`, `true`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprLit {
    pub attrs: Attributes,
    pub lit: Lit,
}

impl Spanner for ExprLit {
    fn span(&self) -> Span {
        let start = if let Some(a) = self.attrs.first() {
            a.span()
        } else {
            self.lit.span()
        };
        start.join(self.lit.span())
    }
}

impl ExprLit {
    /// Returns `true` when the stream is positioned at an identifier `true` or `false`.
    pub fn is_bool_ident(stream: &mut ParseStream) -> bool {
        matches!(stream.curr(), Some(tt) if tt.text() == Some("true") || tt.text() == Some("false"))
    }

    /// Returns `true` when the given token tree is a literal token.
    pub fn is_literal(tt: &TokenTree) -> bool {
        matches!(tt, TokenTree::Literal(_))
    }

    pub fn into_primary_expr(self) -> super::PrimaryExpr {
        super::PrimaryExpr::from(self)
    }
}

impl ToTokens for ExprLit {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.lit.to_tokens(t);
    }
}
