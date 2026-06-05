use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::And;
use moxy_token::{Parse, Span, Spanner, ToTokens, TokenStream};

use super::Type;
use crate::{Lifetime, Mutability};

#[doc = "A reference type (e.g. `&'a T`, `&mut T`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TypeReference {
    pub span: Span,
    pub lifetime: Option<Lifetime>,
    pub mutability: Mutability,
    pub elem: Box<Type>,
}

impl Parse for TypeReference {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let start = stream.span();
        let _ = stream.parse::<And>()?;
        let lifetime = stream.parse::<Option<Lifetime>>()?;
        let mutability = stream.parse::<Mutability>()?;
        let elem = Box::new(stream.parse::<Type>()?);
        Ok(Self {
            span: start,
            lifetime,
            mutability,
            elem,
        })
    }
}

impl Spanner for TypeReference {
    fn span(&self) -> Span {
        self.span.join(self.elem.span())
    }
}

impl ToTokens for TypeReference {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        And::default().to_tokens(tokens);
        self.lifetime.to_tokens(tokens);
        self.mutability.to_tokens(tokens);
        self.elem.to_tokens(tokens);
    }
}
