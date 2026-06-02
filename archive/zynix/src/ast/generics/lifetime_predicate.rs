use crate::ast::{Lifetime, Punctuated};
use crate::parse::{ParseError, ParseStream};
use crate::token::ToTokens;
use crate::token::punct::{Colon, Plus};
use crate::{Parse, Span, TokenStream};

#[doc = "A predicate in a `where` clause."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct LifetimePredicate {
    pub span: Span,
    pub lifetime: Lifetime,
    pub bounds: Punctuated<Lifetime, Plus>,
}

impl Parse for LifetimePredicate {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let lifetime = stream.parse::<Lifetime>()?;
        let bounds = Lifetime::parse_bounds(stream)?;
        Ok(Self {
            span: Span::default(),
            lifetime,
            bounds,
        })
    }
}

impl ToTokens for LifetimePredicate {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.lifetime.to_tokens(t);
        if !self.bounds.is_empty() {
            Colon::default().to_tokens(t);
            self.bounds.to_tokens(t);
        }
    }
}
