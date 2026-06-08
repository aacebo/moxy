use moxy_token::keyword::{Mut, Ref};
use moxy_token::punct::At;
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A pattern that binds a name, optionally with `ref`/`mut` and a subpattern (`@ pat`).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatIdent {
    pub attrs: Vec<Attribute>,
    pub by_ref: Option<Ref>,
    pub mutability: Mutability,
    pub ident: Ident,
    pub subpat: Option<(At, Box<Pattern>)>,
}

impl Spanner for PatIdent {
    fn span(&self) -> Span {
        let start = if let Some(a) = self.attrs.first() {
            a.span()
        } else if let Some(r) = &self.by_ref {
            r.span()
        } else if !matches!(self.mutability, Mutability::Immutable) {
            self.mutability.span()
        } else {
            self.ident.span
        };
        let end = if let Some((_, sub)) = &self.subpat {
            sub.span()
        } else {
            self.ident.span
        };
        start.join(end)
    }
}

impl ToTokens for PatIdent {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }

        self.by_ref.to_tokens(t);
        self.mutability.to_tokens(t);
        self.ident.to_tokens(t);

        if let Some((at, sub)) = &self.subpat {
            at.to_tokens(t);
            sub.to_tokens(t);
        }
    }
}
