use crate::ast::*;
use crate::parse::ParseStream;
use crate::token::{ToTokens, Token, TokenTree};
use crate::{Span, TokenStream};

#[doc = "A literal expression: `1`, `\"hello\"`, `true`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprLit {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub lit: Lit,
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
