use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::Comma;
use moxy_token::{Parse, Span, Spanner, ToTokens, TokenStream};

use super::FieldDef;
use crate::{Delimited, Punctuated};

/// Named struct fields (`{ a: A, b: B }`).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct FieldsNamed {
    pub fields: Delimited<Punctuated<FieldDef, Comma>>,
}

impl Parse for FieldsNamed {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let fields = Delimited::parse_brace_with(stream, Punctuated::parse_terminated)?;
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
