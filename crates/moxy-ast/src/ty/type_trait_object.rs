use moxy_token::keyword::Dyn;
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::Plus;
use moxy_token::{Parse, Span, ToTokens, TokenStream};

use crate::{Punctuated, TypeBound};

#[doc = "A trait object type (e.g. `dyn Iterator<Item = u8>`, `dyn Fn() + 'a`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TypeTraitObject {
    pub span: Span,
    pub dyn_token: Option<Dyn>,
    pub bounds: Punctuated<TypeBound, Plus>,
}

impl Parse for TypeTraitObject {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let dyn_token = stream.parse_if::<Dyn>();
        let bounds = crate::TypeBound::parse_bounds(stream)?;
        Ok(Self {
            span: Span::default(),
            dyn_token,
            bounds,
        })
    }
}

impl ToTokens for TypeTraitObject {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.dyn_token.to_tokens(t);
        self.bounds.to_tokens(t);
    }
}
