use moxy_token::{Span, Spanner, ToTokens};

use crate::*;

/// A type wrapped in an invisible group delimiter (produced during macro expansion).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TypeGroup {
    pub span: Span,
    pub elem: Box<Type>,
}

impl Parse for TypeGroup {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(inner) = cursor.descend(moxy_token::Delim::None) else {
            return false;
        };

        let Some(inner) = Type::skip(inner) else {
            return false;
        };

        inner.is_empty()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let (span, inner) = parser.parse_group_spanned(moxy_token::Delim::None)?;
        Ok(Self {
            span: span.span(),
            elem: Box::new(<_ as Parse>::parse(&inner)?),
        })
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Spanner for TypeGroup {
    fn span(&self) -> Span {
        self.span
    }
}

impl ToTokens for TypeGroup {
    fn to_tokens(&self, tokens: &mut moxy_token::TokenStream) {
        self.elem.to_tokens(tokens);
    }
}
