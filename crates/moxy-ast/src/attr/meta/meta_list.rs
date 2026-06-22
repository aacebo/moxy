use moxy_token::{Comma, Span, Spanner, ToTokens, TokenStream};

use crate::{Delimited, Meta, Path, Punctuated};

/// A list-style meta item (`name(tokens)`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct MetaList {
    pub path: Path,
    pub items: Delimited<Punctuated<Meta, Comma>>,
}

impl MetaList {
    pub fn into_meta(self) -> Meta {
        Meta::List(self)
    }
}

impl Spanner for MetaList {
    fn span(&self) -> Span {
        self.path.span().join(self.items.span())
    }
}

impl ToTokens for MetaList {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.path.to_tokens(t);
        self.items.to_tokens(t);
    }
}
