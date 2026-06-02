use moxy_token::parse::{ParseError, ParseStream};
use moxy_token::punct::Comma;
use moxy_token::{Delim, Group, Parse, Span, ToTokens, TokenStream, TokenTree};

use super::FieldDef;
use crate::Punctuated;

#[doc = "Named struct fields (`{ a: A, b: B }`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct FieldsNamed {
    pub span: Span,
    pub fields: Punctuated<FieldDef, Comma>,
}

impl Parse for FieldsNamed {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let group = stream.parse_group(Delim::Brace)?;
        let mut inner = group.parse();
        let fields = Punctuated::parse_terminated(&mut inner)?;
        Ok(Self {
            span: Span::default(),
            fields,
        })
    }
}

impl ToTokens for FieldsNamed {
    fn to_tokens(&self, t: &mut TokenStream) {
        let mut inner = TokenStream::new();
        self.fields.to_tokens(&mut inner);
        t.extend_one(TokenTree::Group(Group::new(Delim::Brace, inner)));
    }
}
