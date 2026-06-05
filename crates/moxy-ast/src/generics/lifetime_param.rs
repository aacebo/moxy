use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::{Colon, Plus};
use moxy_token::{Parse, Span, Spanner, ToTokens, TokenStream};

use crate::{Attribute, Lifetime, Punctuated};

#[doc = "A lifetime parameter (`'a: 'b + 'c`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct LifetimeParam {
    pub attrs: Vec<Attribute>,
    pub lifetime: Lifetime,
    pub colon_punct: Option<Colon>,
    pub bounds: Punctuated<Lifetime, Plus>,
}

impl Parse for LifetimeParam {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse::<Vec<Attribute>>()?;
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
        let start = if let Some(a) = self.attrs.first() {
            a.span()
        } else {
            self.lifetime.span()
        };
        let end = self.bounds.last().map(|b| b.span()).unwrap_or_else(|| self.lifetime.span());
        start.join(end)
    }
}

impl ToTokens for LifetimeParam {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.lifetime.to_tokens(t);

        if !self.bounds.is_empty() {
            if let Some(colon_punct) = &self.colon_punct {
                colon_punct.to_tokens(t);
            }
            self.bounds.to_tokens(t);
        }
    }
}
