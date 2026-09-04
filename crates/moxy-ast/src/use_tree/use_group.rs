use moxy_token::{LexError, Span, Spanner, ToTokens, TokenStream};

use crate::{Delimited, Parse, ParseError, Parser, Punctuated};

use super::UseTree;

/// A braced use group (`{a, b::c}`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct UseGroup {
    pub items: Delimited<Punctuated<UseTree, Token![,]>>,
}

impl Parse for UseGroup {
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let at = parser.span();

        match parser.parse::<UseTree>()? {
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
