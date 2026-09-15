use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// An or-pattern, e.g. `A | B | C`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatOr {
    pub attrs: Attributes,
    pub cases: Punctuated<Pattern, Token![|]>,
}

impl Spanner for PatOr {
    fn span(&self) -> Span {
        let cases = match (self.cases.first(), self.cases.last()) {
            (Some(a), Some(b)) => a.span().join(b.span()),
            _ => Span::call_site(),
        };

        self.attrs.span().join(cases)
    }
}

impl Parse for PatOr {
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.peek::<Pattern>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            attrs: parser.parse()?,
            cases: Punctuated::parse_separated_nonempty(parser)?,
        })
    }
}

impl ToTokens for PatOr {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.cases.to_tokens(t);
    }
}
