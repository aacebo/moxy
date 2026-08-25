use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::{Colon, Plus};
use moxy_token::{Parse, Span, Spanner, ToTokens, TokenStream};

use crate::{Attributes, Lifetime, Punctuated};

/// A lifetime parameter (`'a: 'b + 'c`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct LifetimeParam {
    pub attrs: Attributes,
    pub lifetime: Lifetime,
    pub colon_punct: Option<Colon>,
    pub bounds: Punctuated<Lifetime, Plus>,
}

impl Parse for LifetimeParam {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse::<Attributes>()?;
        let lifetime = stream.parse::<Lifetime>()?;
        let bounds = Lifetime::parse_bounds(stream)?;
        let colon_punct = if !bounds.is_empty() { Some(Colon::default()) } else { None };

        Ok(Self {
            attrs,
            lifetime,
            colon_punct,
            bounds,
        })
    }
}

impl Spanner for LifetimeParam {
    fn span(&self) -> Span {
        let end = self.bounds.last().map(|b| b.span()).unwrap_or_else(|| self.lifetime.span());
        self.attrs.span().join(end)
    }
}

impl ToTokens for LifetimeParam {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.lifetime.to_tokens(t);

        if !self.bounds.is_empty() {
            if let Some(colon_punct) = &self.colon_punct {
                colon_punct.to_tokens(t);
            }
            self.bounds.to_tokens(t);
        }
    }
}

impl LifetimeParam {
    pub fn into_generic_param(self) -> super::GenericParam {
        super::GenericParam::from(self)
    }
}
