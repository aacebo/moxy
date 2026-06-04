use moxy_token::{Span, ToTokens, TokenStream};

use crate::{Delimited, Path};

#[doc = "A list-style meta item (`name(tokens)`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct MetaList {
    pub span: Span,
    pub path: Path,
    pub tokens: Delimited<TokenStream>,
}

impl ToTokens for MetaList {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.path.to_tokens(t);
        self.tokens.to_tokens(t);
    }
}
