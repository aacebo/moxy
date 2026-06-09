use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::{Delimited, Meta, Path};

/// A list-style meta item (`name(tokens)`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct MetaList {
    pub path: Path,
    pub tokens: Delimited<TokenStream>,
}

impl MetaList {
    pub fn get(&self, path: &Path) -> Option<Meta> {
        let meta: Meta = self.tokens.parse().parse_if()?;
        meta.get(path)
    }
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

impl MetaList {
    pub fn into_meta(self) -> Meta {
        Meta::List(self)
    }
}
