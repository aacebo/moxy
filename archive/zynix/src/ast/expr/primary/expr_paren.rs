use crate::ast::Attribute;
use crate::token::{Delim, Group, ToTokens};
use crate::{Span, TokenStream, TokenTree};

#[doc = "A parenthesized expression: `(x + y)`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprParen {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub expr: Box<super::super::Expr>,
}

impl ToTokens for ExprParen {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        let mut inner = TokenStream::new();
        self.expr.to_tokens(&mut inner);
        t.extend_one(TokenTree::Group(Group::new(Delim::Paren, inner)));
    }
}
