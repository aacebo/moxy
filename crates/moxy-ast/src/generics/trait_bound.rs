use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::{Parse, Span, Spanner, ToTokens, TokenStream};

use crate::{BoundLifetimes, BoundPolarity, Path, TraitBoundModifier};

/// A trait bound (`Trait`, `?Sized`, `for<'a> Trait`).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TraitBound {
    pub polarity: BoundPolarity,
    pub lifetimes: Option<BoundLifetimes>,
    pub modifier: TraitBoundModifier,
    pub path: Path,
}

impl Parse for TraitBound {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let polarity = stream.parse::<BoundPolarity>()?;
        let lifetimes = stream.parse::<Option<BoundLifetimes>>()?;
        let modifier = stream.parse::<TraitBoundModifier>()?;
        let path = stream.parse::<Path>()?;
        Ok(Self {
            polarity,
            lifetimes,
            modifier,
            path,
        })
    }
}

impl Spanner for TraitBound {
    fn span(&self) -> Span {
        let start = match &self.polarity {
            BoundPolarity::Negative(t) => t.span(),
            BoundPolarity::Positive => {
                if let Some(l) = &self.lifetimes {
                    l.span()
                } else {
                    match &self.modifier {
                        TraitBoundModifier::Maybe(t) => t.span(),
                        TraitBoundModifier::None => self.path.span(),
                    }
                }
            }
        };
        start.join(self.path.span())
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
