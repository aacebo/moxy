use crate::Token;
use crate::{Parse, ParseError, Parser};
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use super::Field;
use crate::{Delimited, Punctuated};

/// Named struct fields (`{ a: A, b: B }`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct FieldsNamed {
    pub fields: Delimited<Punctuated<Field, Token![,]>>,
}

impl Parse for FieldsNamed {
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
