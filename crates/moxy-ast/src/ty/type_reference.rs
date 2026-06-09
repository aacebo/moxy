use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::And;
use moxy_token::{Parse, Span, Spanner, ToTokens, TokenStream};

use super::Type;
use crate::{Lifetime, Mutability};

/// A reference type (e.g. `&'a T`, `&mut T`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TypeReference {
    pub and: And,
    pub lifetime: Option<Lifetime>,
    pub mutability: Mutability,
    pub elem: Box<Type>,
}

impl Parse for TypeReference {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let and = stream.parse::<And>()?;
        let lifetime = stream.parse::<Option<Lifetime>>()?;
        let mutability = stream.parse::<Mutability>()?;
        let elem = Box::new(stream.parse::<Type>()?);
        Ok(Self {
            and,
            lifetime,
            mutability,
            elem,
        })
    }
}

impl Spanner for TypeReference {
    fn span(&self) -> Span {
        self.and.span().join(self.elem.span())
    }
}

impl ToTokens for TypeReference {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.and.to_tokens(tokens);
        self.lifetime.to_tokens(tokens);
        self.mutability.to_tokens(tokens);
        self.elem.to_tokens(tokens);
    }
}
