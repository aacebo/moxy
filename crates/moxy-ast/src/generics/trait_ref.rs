use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::{Parse, Span, Spanner, ToTokens, TokenStream};

use crate::{BoundPolarity, Path};

#[doc = "A trait reference (`Trait`, `!Trait`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TraitRef {
    pub span: Span,
    pub polarity: BoundPolarity,
    pub path: Path,
}

impl Parse for TraitRef {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let start = stream.span();
        let polarity = stream.parse::<BoundPolarity>()?;
        let path = stream.parse::<Path>()?;
        Ok(Self {
            span: start.join(path.span),
            polarity,
            path,
        })
    }
}

impl Spanner for TraitRef {
    fn span(&self) -> Span {
        self.span
    }
}

impl ToTokens for TraitRef {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.polarity.to_tokens(tokens);
        self.path.to_tokens(tokens);
    }
}
