use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::PathSep;
use moxy_token::{LexError, Parse, Span, Spanner, ToTokens, TokenStream};

use super::UseTree;
use crate::Ident;

/// A use path segment (`foo::<rest>`).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct UsePath {
    pub ident: Ident,
    pub path_sep: PathSep,
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
        self.ident.span.join(self.tree.span())
    }
}

impl ToTokens for UsePath {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.ident.to_tokens(t);
        self.path_sep.to_tokens(t);
        self.tree.to_tokens(t);
    }
}
