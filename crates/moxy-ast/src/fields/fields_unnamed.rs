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
    pub paren: Delimited<Punctuated<FieldDef, Comma>>,
}

impl Parse for FieldsUnnamed {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let paren = Delimited::parse_paren_with(stream, Punctuated::parse_terminated)?;
        Ok(Self {
            span: Span::default(),
            paren,
        })
    }
}

impl ToTokens for FieldsUnnamed {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.paren.to_tokens(t);
    }
}
