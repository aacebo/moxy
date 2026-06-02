use crate::ast::*;
use crate::token::ToTokens;
use crate::{Span, TokenStream};

#[doc = "A path expression: `std::mem::swap`, `<T as Trait>::assoc`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprPath {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub qself: Option<QSelf>,
    pub path: Path,
}

impl ToTokens for ExprPath {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.path.to_tokens(t);
    }
}
