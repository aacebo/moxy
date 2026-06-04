use moxy_token::parse::{ParseError, ParseStream};
use moxy_token::punct::Comma;
use moxy_token::{Parse, Span, ToTokens, TokenStream};

use super::FieldDef;
use crate::{Delimited, Punctuated};

#[doc = "Named struct fields (`{ a: A, b: B }`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct FieldsNamed {
    pub span: Span,
    pub brace: Delimited<Punctuated<FieldDef, Comma>>,
}

impl Parse for FieldsNamed {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let brace = Delimited::parse_brace_with(stream, Punctuated::parse_terminated)?;
        Ok(Self {
            span: Span::default(),
            brace,
        })
    }
}

impl ToTokens for FieldsNamed {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.brace.to_tokens(t);
    }
}
