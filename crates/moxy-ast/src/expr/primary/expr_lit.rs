use moxy_token::parser::ParseStream;
use moxy_token::{Span, Spanner, ToTokens, Token, TokenStream, TokenTree};

use crate::*;

#[doc = "A literal expression: `1`, `\"hello\"`, `true`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprLit {
    pub attrs: Vec<Attribute>,
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
        matches!(stream.curr(), Some(tt) if tt.name().as_deref() == Some("true") || tt.name().as_deref() == Some("false"))
    }

    /// Returns `true` when the given token tree is a literal token.
    pub fn is_literal(tt: &TokenTree) -> bool {
        matches!(tt, TokenTree::Token(Token::Literal(_)))
    }
}

impl ToTokens for ExprLit {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.lit.to_tokens(t);
    }
}
