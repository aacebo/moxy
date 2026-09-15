use moxy_token::{Delim, Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// Named struct fields (`{ a: A, b: B }`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct FieldsNamed {
    pub fields: Delimited<Punctuated<fields::Field, Token![,]>>,
}

impl Parse for FieldsNamed {
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor
            .descend(Delim::Brace)
            .map(|c| c.peek::<fields::Field>())
            .unwrap_or_default()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let fields = Delimited::parse_brace_with(parser, Punctuated::parse_terminated)?;
        Ok(Self { fields })
    }
}

impl Spanner for FieldsNamed {
    fn span(&self) -> Span {
        self.fields.span()
    }
}

impl ToTokens for FieldsNamed {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.fields.to_tokens(t);
    }
}
