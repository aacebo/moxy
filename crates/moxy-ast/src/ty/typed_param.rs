use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::Colon;
use moxy_token::{Parse, Span, Spanner, ToTokens, TokenStream};

use super::Type;
use crate::{Attributes, Pattern};

/// A typed function parameter (`pat: Type`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TypedParam {
    pub attrs: Attributes,
    pub pat: Box<Pattern>,
    pub colon: Colon,
    pub ty: Box<Type>,
}

impl Parse for TypedParam {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse::<Attributes>()?;
        let pat = Box::new(stream.parse::<Pattern>()?);
        let colon = stream.parse::<Colon>()?;
        let ty = Box::new(stream.parse::<Type>()?);
        Ok(Self { attrs, pat, colon, ty })
    }
}

impl Spanner for TypedParam {
    fn span(&self) -> Span {
        self.attrs.span().join(self.ty.span())
    }
}

impl ToTokens for TypedParam {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.attrs.to_tokens(tokens);
        self.pat.to_tokens(tokens);
        self.colon.to_tokens(tokens);
        self.ty.to_tokens(tokens);
    }
}
