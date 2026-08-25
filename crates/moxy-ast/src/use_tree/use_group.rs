use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::Comma;
use moxy_token::{LexError, Parse, Span, Spanner, ToTokens, TokenStream};

use super::UseTree;
use crate::{Delimited, Punctuated};

/// A braced use group (`{a, b::c}`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct UseGroup {
    pub items: Delimited<Punctuated<UseTree, Comma>>,
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

impl Spanner for UseGroup {
    fn span(&self) -> Span {
        self.items.span()
    }
}

impl ToTokens for UseGroup {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.items.to_tokens(t);
    }
}

impl UseGroup {
    pub fn into_use_tree(self) -> super::UseTree {
        super::UseTree::Group(self)
    }
}
