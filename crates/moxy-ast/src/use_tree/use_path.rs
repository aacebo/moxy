use moxy_token::Token;
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::{LexError, Parse, Span, Spanner, ToTokens, TokenStream};

use super::UseTree;
use crate::Ident;

/// A use path segment (`foo::<rest>`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct UsePath {
    pub prefix: Option<Token![::]>,
    pub ident: Ident,
    pub path_sep: Token![::],
    pub tree: Box<UseTree>,
}

impl Parse for UsePath {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let at = stream.span();

        match UseTree::parse(stream)? {
            UseTree::Path(v) => Ok(v),
            _ => Err(LexError::new(at).message("expected use path").into()),
        }
    }
}

impl Spanner for UsePath {
    fn span(&self) -> Span {
        self.ident.span().join(self.tree.span())
    }
}

impl ToTokens for UsePath {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.prefix.to_tokens(t);
        self.ident.to_tokens(t);
        self.path_sep.to_tokens(t);
        self.tree.to_tokens(t);
    }
}

impl UsePath {
    pub fn into_use_tree(self) -> super::UseTree {
        super::UseTree::Path(self)
    }
}
