use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::{LexError, Parse, Span, Spanner, ToTokens, TokenStream};

use super::UseTree;
use crate::Ident;

/// A leaf name in a use tree (`foo`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct UseName {
    pub ident: Ident,
}

impl Parse for UseName {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let at = stream.span();

        match stream.parse::<UseTree>()? {
            UseTree::Name(v) => Ok(v),
            _ => Err(LexError::new(at).message("expected use name").into()),
        }
    }
}

impl Spanner for UseName {
    fn span(&self) -> Span {
        self.ident.span()
    }
}

impl ToTokens for UseName {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.ident.to_tokens(t);
    }
}

impl UseName {
    pub fn into_use_tree(self) -> super::UseTree {
        super::UseTree::Name(self)
    }
}
