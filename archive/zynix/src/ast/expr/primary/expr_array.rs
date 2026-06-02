use crate::ast::*;
use crate::token::punct::Comma;
use crate::token::{Delim, Group, ToTokens};
use crate::{Span, TokenStream, TokenTree};

#[doc = "An array expression: `[a, b, c]`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprArray {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub elems: Punctuated<super::super::Expr, Comma>,
}

impl ToTokens for ExprArray {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        let mut inner = TokenStream::new();
        self.elems.to_tokens(&mut inner);
        t.extend_one(TokenTree::Group(Group::new(Delim::Bracket, inner)));
    }
}
