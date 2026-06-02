use moxy_token::parse::{ParseError, ParseStream};
use moxy_token::punct::Semi;
use moxy_token::{Delim, Parse, Span, ToTokens, TokenStream};

use super::Type;
use crate::Expr;

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
        tokens.extend_one(moxy_token::TokenTree::Group(moxy_token::Group::new(Delim::Bracket, inner)));
    }
}
