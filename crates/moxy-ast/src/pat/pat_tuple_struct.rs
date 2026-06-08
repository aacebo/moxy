use moxy_token::punct::Comma;
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A tuple-struct pattern, e.g. `Point(x, y)`.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatTupleStruct {
    pub attrs: Vec<Attribute>,
    pub qself: Option<QSelf>,
    pub path: Path,
    pub elems: Delimited<Punctuated<Pattern, Comma>>,
}

impl Spanner for PatTupleStruct {
    fn span(&self) -> Span {
        let start = if let Some(a) = self.attrs.first() {
            a.span()
        } else if let Some(q) = &self.qself {
            q.span()
        } else {
            self.path.span()
        };
        start.join(self.elems.span())
    }
}

impl ToTokens for PatTupleStruct {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.path.to_tokens(t);
        self.elems.to_tokens(t);
    }
}
