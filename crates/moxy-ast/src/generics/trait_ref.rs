use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A trait reference (`Trait`, `!Trait`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TraitRef {
    pub polarity: BoundPolarity,
    pub path: Path,
}

impl Parse for TraitRef {
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.peek::<Token![!]>() || cursor.peek::<Path>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            polarity: parser.parse()?,
            path: parser.parse()?,
        })
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
