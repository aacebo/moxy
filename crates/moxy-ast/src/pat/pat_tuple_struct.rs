use moxy_token::punct::Comma;
use moxy_token::{Paren, Span, ToTokens, TokenStream};

use crate::*;

#[doc = "A tuple-struct pattern, e.g. `Point(x, y)`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatTupleStruct {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub qself: Option<QSelf>,
    pub path: Path,
    pub paren: Paren,
    pub elems: Punctuated<Pattern, Comma>,
}

impl ToTokens for PatTupleStruct {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }

        self.path.to_tokens(t);
        let mut inner = TokenStream::new();
        self.elems.to_tokens(&mut inner);
        self.paren.surround(t, inner);
    }
}
