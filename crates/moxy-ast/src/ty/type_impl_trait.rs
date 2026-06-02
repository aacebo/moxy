use moxy_token::keyword::Impl;
use moxy_token::parse::{ParseError, ParseStream};
use moxy_token::punct::Plus;
use moxy_token::{Parse, Span, ToTokens, TokenStream};

use crate::{Punctuated, TypeBound};

#[doc = "An `impl Trait` type (e.g. `impl Iterator<Item = u8>`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TypeImplTrait {
    pub span: Span,
    pub bounds: Punctuated<TypeBound, Plus>,
}

impl Parse for TypeImplTrait {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let _ = stream.parse::<Impl>()?;
        let bounds = crate::TypeBound::parse_bounds(stream)?;
        Ok(Self {
            span: Span::default(),
            bounds,
        })
    }
}

impl ToTokens for TypeImplTrait {
    fn to_tokens(&self, t: &mut TokenStream) {
        Impl::default().to_tokens(t);
        self.bounds.to_tokens(t);
    }
}
