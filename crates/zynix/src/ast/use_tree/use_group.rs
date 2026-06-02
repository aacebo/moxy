use super::UseTree;
use crate::ast::Punctuated;
use crate::parse::{ParseError, ParseStream};
use crate::token::punct::Comma;
use crate::token::{Delim, Group, LexError, ToTokens};
use crate::{Parse, Span, TokenStream, TokenTree};

#[doc = "A braced use group (`{a, b::c}`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct UseGroup {
    pub span: Span,
    pub items: Punctuated<UseTree, Comma>,
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
        let mut inner = TokenStream::new();
        self.items.to_tokens(&mut inner);
        t.extend_one(TokenTree::Group(Group::new(Delim::Brace, inner)));
    }
}
