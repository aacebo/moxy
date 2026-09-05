use crate::Token;
use crate::{Parse, ParseError, Parser};
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use super::Field;
use crate::{Delimited, Punctuated};

/// Tuple-struct fields (`(A, B)`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct FieldsUnnamed {
    pub fields: Delimited<Punctuated<Field, Token![,]>>,
}

impl Parse for FieldsUnnamed {
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let fields = Delimited::parse_paren_with(parser, Punctuated::parse_terminated)?;
        Ok(Self { fields })
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
