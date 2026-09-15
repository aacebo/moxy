use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A lifetime parameter (`'a: 'b + 'c`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct LifetimeParam {
    pub attrs: Attributes,
    pub lifetime: Lifetime,
    pub colon_punct: Option<Token![:]>,
    pub bounds: Punctuated<Lifetime, Token![+]>,
}

impl Parse for LifetimeParam {
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.peek::<Lifetime>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            attrs: parser.parse()?,
            lifetime: parser.parse()?,
            colon_punct: parser.parse()?,
            bounds: parser.parse()?,
        })
    }
}

impl Spanner for LifetimeParam {
    fn span(&self) -> Span {
        let end = self.bounds.last().map(|b| b.span()).unwrap_or_else(|| self.lifetime.span());
        self.attrs.span().join(end)
    }
}

impl ToTokens for LifetimeParam {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.lifetime.to_tokens(t);
        self.colon_punct.to_tokens(t);
        self.bounds.to_tokens(t);
    }
}
