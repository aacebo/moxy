use crate::ast::{Attribute, Lifetime, Punctuated};
use crate::parse::{ParseError, ParseStream};
use crate::token::ToTokens;
use crate::token::punct::{Colon, Plus};
use crate::{Parse, Span, TokenStream};

#[doc = "A lifetime parameter (`'a: 'b + 'c`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct LifetimeParam {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub lifetime: Lifetime,
    pub bounds: Punctuated<Lifetime, Plus>,
}

impl Parse for LifetimeParam {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse_vec::<Attribute>()?;
        let lifetime = stream.parse::<Lifetime>()?;
        let bounds = Lifetime::parse_bounds(stream)?;
        Ok(Self {
            span: Span::default(),
            attrs,
            lifetime,
            bounds,
        })
    }
}

impl ToTokens for LifetimeParam {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.lifetime.to_tokens(t);

        if !self.bounds.is_empty() {
            Colon::default().to_tokens(t);
            self.bounds.to_tokens(t);
        }
    }
}
