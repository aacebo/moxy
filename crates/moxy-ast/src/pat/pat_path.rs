use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

#[doc = "A path pattern, e.g. `Some` or `std::option::Option::None`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatPath {
    pub attrs: Vec<Attribute>,
    pub qself: Option<QSelf>,
    pub path: Path,
}

impl Spanner for PatPath {
    fn span(&self) -> Span {
        let start = if let Some(a) = self.attrs.first() {
            a.span()
        } else if let Some(q) = &self.qself {
            q.span()
        } else {
            self.path.span()
        };
        start.join(self.path.span())
    }
}

impl ToTokens for PatPath {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }

        self.path.to_tokens(t);
    }
}
