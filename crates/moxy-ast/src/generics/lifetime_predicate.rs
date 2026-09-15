use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A predicate in a `where` clause.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct LifetimePredicate {
    pub lifetime: Lifetime,
    pub colon_punct: Token![:],
    pub bounds: Punctuated<Lifetime, Token![+]>,
}

impl Parse for LifetimePredicate {
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.peek::<Lifetime>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            lifetime: parser.parse()?,
            colon_punct: parser.parse()?,
            bounds: parser.parse()?,
        })
    }
}

impl Spanner for LifetimePredicate {
    fn span(&self) -> Span {
        let end = self.bounds.last().map(|b| b.span()).unwrap_or_else(|| self.lifetime.span());
        self.lifetime.span().join(end)
    }
}

impl ToTokens for LifetimePredicate {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.lifetime.to_tokens(t);
        self.colon_punct.to_tokens(t);
        self.bounds.to_tokens(t);
    }
}
