use crate::ast::Attribute;
use crate::token::punct::Semi;
use crate::token::{Delim, Group, ToTokens};
use crate::{Span, TokenStream, TokenTree};

#[doc = "A repeat expression: `[0u8; 16]`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprRepeat {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub elem: Box<super::super::Expr>,
    pub len: Box<super::super::Expr>,
}

impl ToTokens for ExprRepeat {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        let mut inner = TokenStream::new();
        self.elem.to_tokens(&mut inner);
        Semi::default().to_tokens(&mut inner);
        self.len.to_tokens(&mut inner);
        t.extend_one(TokenTree::Group(Group::new(Delim::Bracket, inner)));
    }
}
