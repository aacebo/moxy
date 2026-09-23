use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A trait bound (`Trait`, `?Sized`, `for<'a> Trait`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TraitBound {
    pub polarity: Option<Token![!]>,
    pub lifetimes: Option<BoundLifetimes>,
    pub modifier: Option<Token![?]>,
    pub path: Path,
}

impl Parse for TraitBound {
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.peek::<Token![!]>() || cursor.peek::<Token![for]>() || cursor.peek::<Token![?]>() || cursor.peek::<Path>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            polarity: parser.parse()?,
            lifetimes: parser.parse()?,
            modifier: parser.parse()?,
            path: parser.parse()?,
        })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = cursor.skip::<Option<Token![!]>>()?;
        cursor = cursor.skip::<Option<BoundLifetimes>>()?;
        cursor = cursor.skip::<Option<Token![?]>>()?;
        cursor.skip::<Path>()
    }
}

impl Spanner for TraitBound {
    fn span(&self) -> Span {
        let start = self
            .polarity
            .as_ref()
            .map(Spanner::span)
            .or_else(|| self.lifetimes.as_ref().map(Spanner::span))
            .or_else(|| self.modifier.as_ref().map(Spanner::span))
            .unwrap_or_else(|| self.path.span());

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
