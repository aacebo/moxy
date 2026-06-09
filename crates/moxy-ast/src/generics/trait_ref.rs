use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::{Parse, Span, Spanner, ToTokens, TokenStream};

use crate::{BoundPolarity, Path};

/// A trait reference (`Trait`, `!Trait`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TraitRef {
    pub polarity: BoundPolarity,
    pub path: Path,
}

impl Parse for TraitRef {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let polarity = stream.parse::<BoundPolarity>()?;
        let path = stream.parse::<Path>()?;
        Ok(Self { polarity, path })
    }
}

impl Spanner for TraitRef {
    fn span(&self) -> Span {
        let start = match &self.polarity {
            BoundPolarity::Negative(t) => t.span(),
            BoundPolarity::Positive => self.path.span(),
        };
        start.join(self.path.span())
    }
}

impl ToTokens for TraitRef {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.polarity.to_tokens(tokens);
        self.path.to_tokens(tokens);
    }
}
