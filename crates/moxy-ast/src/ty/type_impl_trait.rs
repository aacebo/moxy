use moxy_token::keyword::Impl;
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::Plus;
use moxy_token::{Parse, Span, Spanner, ToTokens, TokenStream};

use crate::{Punctuated, TypeBound};

#[doc = "An `impl Trait` type (e.g. `impl Iterator<Item = u8>`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TypeImplTrait {
    pub impl_keyword: Impl,
    pub bounds: Punctuated<TypeBound, Plus>,
}

impl Parse for TypeImplTrait {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let impl_keyword = stream.parse::<Impl>()?;
        let bounds = crate::TypeBound::parse_bounds(stream)?;
        Ok(Self { impl_keyword, bounds })
    }
}

impl Spanner for TypeImplTrait {
    fn span(&self) -> Span {
        let end = self
            .bounds
            .last()
            .map(|b| b.span())
            .unwrap_or_else(|| self.impl_keyword.span());
        self.impl_keyword.span().join(end)
    }
}

impl ToTokens for TypeImplTrait {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.impl_keyword.to_tokens(t);
        self.bounds.to_tokens(t);
    }
}
