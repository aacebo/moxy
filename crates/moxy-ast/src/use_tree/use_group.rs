use moxy_token::parse::{ParseError, ParseStream};
use moxy_token::punct::Comma;
use moxy_token::{LexError, Parse, Span, ToTokens, TokenStream};

use super::UseTree;
use crate::{Delimited, Punctuated};

#[doc = "A braced use group (`{a, b::c}`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct UseGroup {
    pub span: Span,
    pub brace: Delimited<Punctuated<UseTree, Comma>>,
}

impl Parse for UseGroup {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let at = stream.span();
        match UseTree::parse(stream)? {
            UseTree::Group(v) => Ok(v),
            _ => Err(LexError::new(at).message("expected use group").into()),
        }
    }
}

impl ToTokens for UseGroup {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.brace.to_tokens(t);
    }
}
