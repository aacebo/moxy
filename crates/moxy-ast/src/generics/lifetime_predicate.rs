use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::{Colon, Plus};
use moxy_token::{Parse, Span, Spanner, ToTokens, TokenStream};

use crate::{Lifetime, Punctuated};

/// A predicate in a `where` clause.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct LifetimePredicate {
    pub lifetime: Lifetime,
    pub colon_punct: Colon,
    pub bounds: Punctuated<Lifetime, Plus>,
}

impl Parse for LifetimePredicate {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let lifetime = stream.parse::<Lifetime>()?;
        let bounds = Lifetime::parse_bounds(stream)?;
        Ok(Self {
            lifetime,
            colon_punct: Colon::default(),
            bounds,
        })
    }
}

impl Spanner for LifetimePredicate {
    fn span(&self) -> Span {
        let end = self.bounds.last().map(|b| b.span()).unwrap_or_else(|| self.lifetime.span());
        self.lifetime.span().join(end)
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
