use moxy_token::punct::Colon;
use moxy_token::{Span, ToTokens, TokenStream};

use crate::*;

#[doc = "A type-ascription pattern, e.g. `x: i32`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatType {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub pat: Box<Pattern>,
    pub ty: Box<Type>,
}

impl ToTokens for PatType {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.pat.to_tokens(t);
        Colon::default().to_tokens(t);
        self.ty.to_tokens(t);
    }
}
