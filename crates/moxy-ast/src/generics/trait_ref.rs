use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A trait reference (`Trait`, `!Trait`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TraitRef {
    pub polarity: Option<Token![!]>,
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

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor.skip::<Option<Token![!]>>()?.skip::<Path>()
    }
}

impl Spanner for TraitRef {
    fn span(&self) -> Span {
        let start = self.polarity.as_ref().map_or_else(|| self.path.span(), Spanner::span);

        start.join(self.path.span())
    }
}

impl ToTokens for TraitRef {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.polarity.to_tokens(tokens);
        self.path.to_tokens(tokens);
    }
}
