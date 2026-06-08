use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::Comma;
use moxy_token::{Parse, Span, Spanner, ToTokens, TokenStream};

use super::FieldDef;
use crate::{Delimited, Punctuated};

/// Tuple-struct fields (`(A, B)`).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct FieldsUnnamed {
    pub fields: Delimited<Punctuated<FieldDef, Comma>>,
}

impl Parse for FieldsUnnamed {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let fields = Delimited::parse_paren_with(stream, Punctuated::parse_terminated)?;
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
