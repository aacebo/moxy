use crate::Token;
use crate::{Parse, ParseError, Parser};
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::{Lifetime, Punctuated};

/// A predicate in a `where` clause.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct LifetimePredicate {
    pub lifetime: Lifetime,
    pub colon_punct: Token![:],
    pub bounds: Punctuated<Lifetime, Token![+]>,
}

impl Parse for LifetimePredicate {
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let lifetime = parser.parse::<Lifetime>()?;
        let bounds = Lifetime::parse_bounds(parser)?;

        Ok(Self {
            lifetime,
            colon_punct: <Token![:]>::default(),
            bounds,
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

        if !self.bounds.is_empty() {
            self.colon_punct.to_tokens(t);
            self.bounds.to_tokens(t);
        }
    }
}
