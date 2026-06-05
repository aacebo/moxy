use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::{Parse, Span, ToTokens, TokenStream};

use crate::{BoundLifetimes, BoundPolarity, Path, TraitBoundModifier};

#[doc = "A trait bound (`Trait`, `?Sized`, `for<'a> Trait`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TraitBound {
    pub span: Span,
    pub polarity: BoundPolarity,
    pub lifetimes: Option<BoundLifetimes>,
    pub modifier: TraitBoundModifier,
    pub path: Path,
}

impl Parse for TraitBound {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let start = stream.span();
        let polarity = stream.parse::<BoundPolarity>()?;
        let lifetimes = stream.parse::<Option<BoundLifetimes>>()?;
        let modifier = stream.parse::<TraitBoundModifier>()?;
        let path = stream.parse::<Path>()?;
        Ok(Self {
            span: start.join(path.span),
            polarity,
            lifetimes,
            modifier,
            path,
        })
    }
}

impl ToTokens for TraitBound {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.polarity.to_tokens(tokens);
        self.lifetimes.to_tokens(tokens);
        self.modifier.to_tokens(tokens);
        self.path.to_tokens(tokens);
    }
}
