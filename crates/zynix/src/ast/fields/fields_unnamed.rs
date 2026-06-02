use super::FieldDef;
use crate::ast::Punctuated;
use crate::parse::{ParseError, ParseStream};
use crate::token::punct::Comma;
use crate::token::{Delim, Group, ToTokens};
use crate::{Parse, Span, TokenStream, TokenTree};

#[doc = "Tuple-struct fields (`(A, B)`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct FieldsUnnamed {
    pub span: Span,
    pub fields: Punctuated<FieldDef, Comma>,
}

impl Parse for FieldsUnnamed {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let group = stream.parse_group(Delim::Paren)?;
        let mut inner = group.parse();
        let fields = Punctuated::parse_terminated(&mut inner)?;
        Ok(Self {
            span: Span::default(),
            fields,
        })
    }
}

impl ToTokens for FieldsUnnamed {
    fn to_tokens(&self, t: &mut TokenStream) {
        let mut inner = TokenStream::new();
        self.fields.to_tokens(&mut inner);
        t.extend_one(TokenTree::Group(Group::new(Delim::Paren, inner)));
    }
}
