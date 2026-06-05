use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::Colon;
use moxy_token::{Parse, Span, Spanner, ToTokens, TokenStream};

use super::Type;
use crate::{Attribute, Pattern};

#[doc = "A typed function parameter (`pat: Type`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TypedParam {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub pat: Box<Pattern>,
    pub ty: Box<Type>,
}

impl Parse for TypedParam {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let start = stream.span();
        let attrs = stream.parse::<Vec<Attribute>>()?;
        let pat = Box::new(stream.parse::<Pattern>()?);
        let _ = stream.parse::<Colon>()?;
        let ty = Box::new(stream.parse::<Type>()?);
        Ok(Self {
            span: start,
            attrs,
            pat,
            ty,
        })
    }
}

impl Spanner for TypedParam {
    fn span(&self) -> Span {
        let start = if let Some(a) = self.attrs.first() {
            a.span()
        } else {
            self.pat.span()
        };
        start.join(self.ty.span())
    }
}

impl ToTokens for TypedParam {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.attrs.to_tokens(tokens);
        self.pat.to_tokens(tokens);
        Colon::default().to_tokens(tokens);
        self.ty.to_tokens(tokens);
    }
}
