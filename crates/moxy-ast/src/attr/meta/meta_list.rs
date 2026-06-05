use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::{Delimited, Path};

#[doc = "A list-style meta item (`name(tokens)`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct MetaList {
    pub path: Path,
    pub tokens: Delimited<TokenStream>,
}

impl Spanner for MetaList {
    fn span(&self) -> Span {
        self.path.span().join(self.tokens.span())
    }
}

impl ToTokens for MetaList {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.path.to_tokens(t);
        self.tokens.to_tokens(t);
    }
}
