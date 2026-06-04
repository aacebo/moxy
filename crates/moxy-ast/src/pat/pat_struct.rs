use moxy_token::punct::{Comma, DotDot};
use moxy_token::{Brace, Span, ToTokens, TokenStream};

use crate::pat::PatField;
use crate::*;

#[doc = "A struct pattern, e.g. `Point { x, y }` or `Point { x, .. }`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatStruct {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub qself: Option<QSelf>,
    pub path: Path,
    pub brace: Brace,
    pub fields: Punctuated<PatField, Comma>,
    pub rest: Option<DotDot>,
}

impl ToTokens for PatStruct {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }

        self.path.to_tokens(t);
        let mut inner = TokenStream::new();
        self.fields.to_tokens(&mut inner);

        if let Some(dotdot) = &self.rest {
            dotdot.to_tokens(&mut inner);
        }

        self.brace.surround(t, inner);
    }
}
