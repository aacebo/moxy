use moxy_token::parse::{ParseError, ParseStream};
use moxy_token::punct::Comma;
use moxy_token::{Paren, Parse, Span, ToTokens, TokenStream};

use super::FieldDef;
use crate::Punctuated;

#[doc = "Tuple-struct fields (`(A, B)`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct FieldsUnnamed {
    pub span: Span,
    pub paren: Paren,
    pub fields: Punctuated<FieldDef, Comma>,
}

impl Parse for FieldsUnnamed {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let (paren, group) = stream.parse_paren()?;
        let mut inner = group.parse();
        let fields = Punctuated::parse_terminated(&mut inner)?;
        Ok(Self {
            span: Span::default(),
            paren,
            fields,
        })
    }
}

impl ToTokens for FieldsUnnamed {
    fn to_tokens(&self, t: &mut TokenStream) {
        let mut inner = TokenStream::new();
        self.fields.to_tokens(&mut inner);
        self.paren.surround(t, inner);
    }
}
