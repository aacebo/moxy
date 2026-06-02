use crate::ast::*;
use crate::token::ToTokens;
use crate::token::keyword::{Mut, Ref};
use crate::token::punct::At;
use crate::{Span, TokenStream};

#[doc = "A pattern that binds a name, optionally with `ref`/`mut` and a subpattern (`@ pat`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatIdent {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub by_ref: bool,
    pub mutability: Mutability,
    pub ident: Ident,
    pub subpat: Option<Box<Pattern>>,
}

impl ToTokens for PatIdent {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }

        if self.by_ref {
            Ref::default().to_tokens(t);
        }

        self.mutability.to_tokens(t);
        self.ident.to_tokens(t);

        if let Some(sub) = &self.subpat {
            At::default().to_tokens(t);
            sub.to_tokens(t);
        }
    }
}
