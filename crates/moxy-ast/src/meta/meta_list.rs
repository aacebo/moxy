use moxy_token::span::DelimSpan;
use moxy_token::{Delim, Group, Span, ToTokens, TokenStream, TokenTree};

use crate::Path;

#[doc = "A list-style meta item (`name(tokens)`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct MetaList {
    pub span: Span,
    pub path: Path,
    pub delim: Delim,
    pub delim_span: DelimSpan,
    pub tokens: TokenStream,
}

impl ToTokens for MetaList {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.path.to_tokens(t);
        let mut group = Group::new(self.delim, self.tokens.clone());
        group.set_span(self.delim_span);
        t.extend_one(TokenTree::Group(group));
    }
}
