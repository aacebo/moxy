use moxy_token::punct::Comma;
use moxy_token::{Span, ToTokens, TokenStream};

use crate::*;

#[doc = "A tuple-struct pattern, e.g. `Point(x, y)`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatTupleStruct {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub qself: Option<QSelf>,
    pub path: Path,
    pub elems: Delimited<Punctuated<Pattern, Comma>>,
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
