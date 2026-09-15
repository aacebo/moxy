use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A single field binding inside a struct pattern, e.g. `x` (shorthand) or `x: pat`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatField {
    pub attrs: Attributes,
    pub member: Member,
    pub colon: Option<Token![:]>,
    pub pat: Pattern,
}

impl PatField {
    pub fn is_shorthand(&self) -> bool {
        self.colon.is_none()
    }
}

impl Spanner for PatField {
    fn span(&self) -> Span {
        self.attrs.span().join(self.pat.span())
    }
}

impl Parse for PatField {
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.peek::<Member>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            attrs: parser.parse()?,
            member: parser.parse()?,
            colon: parser.parse()?,
            pat: parser.parse()?,
        })
    }
}

impl ToTokens for PatField {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.member.to_tokens(t);
        self.colon.to_tokens(t);
        self.pat.to_tokens(t);
    }
}
