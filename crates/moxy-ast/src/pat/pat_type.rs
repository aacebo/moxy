use moxy_token::Token;
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::{Parse, Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A type-ascription pattern, e.g. `x: i32`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatType {
    pub attrs: Attributes,
    pub pat: Box<Pattern>,
    pub colon: Token![:],
    pub ty: Box<Type>,
}

impl Spanner for PatType {
    fn span(&self) -> Span {
        self.attrs.span().join(self.ty.span())
    }
}

impl ToTokens for PatType {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.pat.to_tokens(t);
        self.colon.to_tokens(t);
        self.ty.to_tokens(t);
    }
}

impl PatType {
    pub fn into_pattern(self) -> super::Pattern {
        super::Pattern::from(self)
    }
}

impl Parse for PatType {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse()?;
        let pat = Box::new(Pattern::parse_single(stream)?);
        let colon = stream.parse()?;
        let ty = Box::new(stream.parse()?);

        Ok(Self { attrs, pat, colon, ty })
    }
}
