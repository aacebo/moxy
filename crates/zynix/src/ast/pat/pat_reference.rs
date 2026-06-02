use crate::ast::*;
use crate::token::ToTokens;
use crate::token::punct::And;
use crate::{Span, TokenStream};

#[doc = "A reference pattern, e.g. `&x` or `&mut x`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatReference {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub mutability: Mutability,
    pub pat: Box<Pattern>,
}

impl ToTokens for PatReference {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        And::default().to_tokens(t);
        self.mutability.to_tokens(t);
        self.pat.to_tokens(t);
    }
}
