use moxy_token::token::{Delim, Group, ToTokens};
use moxy_token::{Span, TokenStream, TokenTree};

use crate::Attribute;

#[doc = "An index expression: `a[0]`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprIndex {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub base: Box<super::super::Expr>,
    pub index: Box<super::super::Expr>,
}

impl ToTokens for ExprIndex {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.base.to_tokens(t);
        let mut inner = TokenStream::new();
        self.index.to_tokens(&mut inner);
        t.extend_one(TokenTree::Group(Group::new(Delim::Bracket, inner)));
    }
}
