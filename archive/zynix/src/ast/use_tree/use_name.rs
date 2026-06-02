use super::UseTree;
use crate::ast::Ident;
use crate::parse::{ParseError, ParseStream};
use crate::token::{LexError, ToTokens};
use crate::{Parse, Span, TokenStream};

#[doc = "A leaf name in a use tree (`foo`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct UseName {
    pub span: Span,
    pub ident: Ident,
}

impl Parse for UseName {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let at = stream.span();
        match UseTree::parse(stream)? {
            UseTree::Name(v) => Ok(v),
            _ => Err(LexError::new(at).message("expected use name").into()),
        }
    }
}

impl ToTokens for UseName {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.ident.to_tokens(t);
    }
}
