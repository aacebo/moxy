use super::Type;
use crate::ast::Expr;
use crate::parse::{ParseError, ParseStream};
use crate::token::punct::Semi;
use crate::token::{Delim, ToTokens};
use crate::{Parse, Span, TokenStream};

#[doc = "A fixed-size array type (`[T; N]`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TypeArray {
    pub span: Span,
    pub elem: Box<Type>,
    pub len: Expr,
}

impl Parse for TypeArray {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let group = stream.parse_group(Delim::Bracket)?;
        let mut inner = group.parse();
        let elem = Box::new(inner.parse::<Type>()?);
        let _ = inner.parse::<Semi>()?;
        let len = inner.parse::<Expr>()?;
        Ok(Self {
            span: Span::default(),
            elem,
            len,
        })
    }
}

impl ToTokens for TypeArray {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut inner = TokenStream::new();
        self.elem.to_tokens(&mut inner);
        Semi::default().to_tokens(&mut inner);
        self.len.to_tokens(&mut inner);
        tokens.extend_one(crate::TokenTree::Group(crate::token::Group::new(Delim::Bracket, inner)));
    }
}
