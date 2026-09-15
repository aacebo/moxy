use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A reference pattern, e.g. `&x` or `&mut x`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatReference {
    pub attrs: Attributes,
    pub and: Token![&],
    pub mutability: Mutability,
    pub pat: Box<Pattern>,
}

impl Spanner for PatReference {
    fn span(&self) -> Span {
        self.attrs.span().join(self.pat.span())
    }
}

impl Parse for PatReference {
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.peek::<Token![&]>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            attrs: parser.parse()?,
            and: parser.parse()?,
            mutability: parser.parse()?,
            pat: parser.parse()?,
        })
    }
}

impl ToTokens for PatReference {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.and.to_tokens(t);
        self.mutability.to_tokens(t);
        self.pat.to_tokens(t);
    }
}
