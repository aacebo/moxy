use moxy_token::{Delim, Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// Tuple-struct fields (`(A, B)`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct FieldsUnnamed {
    pub fields: Delimited<Punctuated<Field, Token![,]>>,
}

impl Parse for FieldsUnnamed {
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.is_delimited(Delim::Paren)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let fields = Delimited::parse_paren_with(parser, Punctuated::parse_terminated)?;
        Ok(Self { fields })
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        let mut inner = cursor.descend(Delim::Paren)?;

        while !inner.is_empty() {
            inner = inner.skip::<Field>()?;

            if inner.is_empty() {
                break;
            }

            inner = inner.skip::<Token![,]>()?;
        }

        Some(cursor.offset(1))
    }
}

impl Spanner for FieldsUnnamed {
    fn span(&self) -> Span {
        self.fields.span()
    }
}

impl ToTokens for FieldsUnnamed {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.fields.to_tokens(t);
    }
}
