use moxy_token::parse::{ParseError, ParseStream};
use moxy_token::punct::Semi;
use moxy_token::{Bracket, Parse, Span, ToTokens, TokenStream};

use super::Type;
use crate::Expr;

#[doc = "A fixed-size array type (`[T; N]`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TypeArray {
    pub span: Span,
    pub bracket: Bracket,
    pub elem: Box<Type>,
    pub semi: Semi,
    pub len: Expr,
}

impl Parse for TypeArray {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let (bracket, group) = stream.parse_bracket()?;
        let mut inner = group.parse();
        let elem = Box::new(inner.parse::<Type>()?);
        let semi = inner.parse::<Semi>()?;
        let len = inner.parse::<Expr>()?;
        Ok(Self {
            span: Span::default(),
            bracket,
            elem,
            semi,
            len,
        })
    }
}

impl ToTokens for TypeArray {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut inner = TokenStream::new();
        self.elem.to_tokens(&mut inner);
        self.semi.to_tokens(&mut inner);
        self.len.to_tokens(&mut inner);
        self.bracket.surround(tokens, inner);
    }
}
