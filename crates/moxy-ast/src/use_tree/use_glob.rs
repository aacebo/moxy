use crate::{Parse, ParseError, Parser};
use moxy_token::Token;
use moxy_token::{LexError, Span, Spanner, ToTokens, TokenStream};

use super::UseTree;

/// A glob import (`*`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct UseGlob {
    pub star: Token![*],
}

impl Parse for UseGlob {
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let at = parser.span();

        match parser.parse::<UseTree>()? {
            UseTree::Glob(v) => Ok(v),
            _ => Err(LexError::new(at).message("expected `*`").into()),
        }
    }
}

impl Spanner for UseGlob {
    fn span(&self) -> Span {
        self.star.span()
    }
}

impl ToTokens for UseGlob {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.star.to_tokens(t);
    }
}

impl UseGlob {
    pub fn into_use_tree(self) -> super::UseTree {
        super::UseTree::Glob(self)
    }
}
