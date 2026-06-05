use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::{Colon, Plus};
use moxy_token::{Parse, Span, ToTokens, TokenStream};

use crate::{Lifetime, Punctuated};

#[doc = "A predicate in a `where` clause."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct LifetimePredicate {
    pub span: Span,
    pub lifetime: Lifetime,
    pub colon_punct: Colon,
    pub bounds: Punctuated<Lifetime, Plus>,
}

impl Parse for LifetimePredicate {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let lifetime = stream.parse::<Lifetime>()?;
        let bounds = Lifetime::parse_bounds(stream)?;
        Ok(Self {
            span: Span::default(),
            lifetime,
            colon_punct: Colon::default(),
            bounds,
        })
    }
}

impl ToTokens for LifetimePredicate {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.lifetime.to_tokens(t);
        if !self.bounds.is_empty() {
            self.colon_punct.to_tokens(t);
            self.bounds.to_tokens(t);
        }
    }
}
