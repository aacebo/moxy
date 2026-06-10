use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::{Meta, Path};

/// A list-style meta item (`name(tokens)`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct MetaCustom {
    pub path: Path,
    pub tokens: TokenStream,
}

impl MetaCustom {
    pub fn into_meta(self) -> Meta {
        Meta::Custom(self)
    }
}

impl Spanner for MetaCustom {
    fn span(&self) -> Span {
        self.path.span().join(self.tokens.span())
    }
}

impl ToTokens for MetaCustom {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.path.to_tokens(t);
        self.tokens.to_tokens(t);
    }
}
