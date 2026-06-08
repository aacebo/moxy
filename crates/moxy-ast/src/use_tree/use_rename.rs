use moxy_token::keyword::As;
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::{LexError, Parse, Span, Spanner, ToTokens, TokenStream};

use super::UseTree;
use crate::Ident;

/// A renamed use leaf (`foo as bar`).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct UseRename {
    pub ident: Ident,
    pub as_keyword: As,
    pub rename: Ident,
}

impl Parse for UseRename {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let at = stream.span();
        match UseTree::parse(stream)? {
            UseTree::Rename(v) => Ok(v),
            _ => Err(LexError::new(at).message("expected use rename").into()),
        }
    }
}

impl Spanner for UseRename {
    fn span(&self) -> Span {
        self.ident.span().join(self.rename.span())
    }
}

impl ToTokens for UseRename {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.ident.to_tokens(t);
        self.as_keyword.to_tokens(t);
        self.rename.to_tokens(t);
    }
}
