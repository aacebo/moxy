use moxy_token::punct::Semi;
use moxy_token::{Bracket, Span, ToTokens, TokenStream};

use crate::Attribute;

#[doc = "A repeat expression: `[0u8; 16]`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprRepeat {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub bracket: Bracket,
    pub elem: Box<super::super::Expr>,
    pub semi: Semi,
    pub len: Box<super::super::Expr>,
}

impl ToTokens for ExprRepeat {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        let mut inner = TokenStream::new();
        self.elem.to_tokens(&mut inner);
        self.semi.to_tokens(&mut inner);
        self.len.to_tokens(&mut inner);
        self.bracket.surround(t, inner);
    }
}
