use moxy_token::parse::{ParseError, ParseStream};
use moxy_token::punct::Comma;
use moxy_token::{Brace, Parse, Span, ToTokens, TokenStream};

use super::FieldDef;
use crate::Punctuated;

#[doc = "Named struct fields (`{ a: A, b: B }`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct FieldsNamed {
    pub span: Span,
    pub brace: Brace,
    pub fields: Punctuated<FieldDef, Comma>,
}

impl Parse for FieldsNamed {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let (brace, group) = stream.parse_brace()?;
        let mut inner = group.parse();
        let fields = Punctuated::parse_terminated(&mut inner)?;
        Ok(Self {
            span: Span::default(),
            brace,
            fields,
        })
    }
}

impl ToTokens for FieldsNamed {
    fn to_tokens(&self, t: &mut TokenStream) {
        let mut inner = TokenStream::new();
        self.fields.to_tokens(&mut inner);
        self.brace.surround(t, inner);
    }
}
