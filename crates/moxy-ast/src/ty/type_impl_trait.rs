use crate::{Parse, ParseError, Parser};
use moxy_token::Token;
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::{Punctuated, TypeBound};

/// An `impl Trait` type (e.g. `impl Iterator<Item = u8>`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TypeImplTrait {
    pub impl_keyword: Token![impl],
    pub bounds: Punctuated<TypeBound, Token![+]>,
}

impl Parse for TypeImplTrait {
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let impl_keyword = parser.parse::<Token![impl]>()?;
        let bounds = crate::TypeBound::parse_bounds(parser)?;
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
