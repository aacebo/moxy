use moxy_token::parse::{ParseError, ParseStream};
use moxy_token::punct::Comma;
use moxy_token::{Parse, Span, ToTokens, TokenStream};

use super::FieldDef;
use crate::{Delimited, Punctuated};

#[doc = "Tuple-struct fields (`(A, B)`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct FieldsUnnamed {
    pub span: Span,
    pub fields: Delimited<Punctuated<FieldDef, Comma>>,
}

impl Parse for FieldsUnnamed {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let fields = Delimited::parse_paren_with(stream, Punctuated::parse_terminated)?;
        Ok(Self {
            span: Span::default(),
            fields,
        })
    }
}

impl ToTokens for FieldsUnnamed {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.fields.to_tokens(t);
    }
}
