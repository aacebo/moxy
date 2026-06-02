use moxy_token::token::{Delim, Group, ToTokens};
use moxy_token::{Span, TokenStream, TokenTree};

use crate::Path;

#[doc = "A list-style meta item (`name(tokens)`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct MetaList {
    pub span: Span,
    pub path: Path,
    pub delim: Delim,
    pub tokens: TokenStream,
}

impl ToTokens for MetaList {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.path.to_tokens(t);
        t.extend_one(TokenTree::Group(Group::new(self.delim, self.tokens.clone())));
    }
}
