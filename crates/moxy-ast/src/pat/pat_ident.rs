use moxy_token::keyword::Ref;
use moxy_token::punct::At;
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A pattern that binds a name, optionally with `ref`/`mut` and a subpattern (`@ pat`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatIdent {
    pub attrs: Attributes,
    pub by_ref: Option<Ref>,
    pub mutability: Mutability,
    pub ident: Ident,
    pub subpat: Option<(At, Box<Pattern>)>,
}

impl Spanner for PatIdent {
    fn span(&self) -> Span {
        let end = if let Some((_, sub)) = &self.subpat {
            sub.span()
        } else {
            self.ident.span()
        };

        self.attrs.span().join(end)
    }
}

impl ToTokens for PatIdent {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.by_ref.to_tokens(t);
        self.mutability.to_tokens(t);
        self.ident.to_tokens(t);

        if let Some((at, sub)) = &self.subpat {
            at.to_tokens(t);
            sub.to_tokens(t);
        }
    }
}

impl PatIdent {
    pub fn into_pattern(self) -> super::Pattern {
        super::Pattern::from(self)
    }
}
