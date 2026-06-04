use moxy_token::punct::{Comma, DotDot};
use moxy_token::{Span, ToTokens, TokenStream};

use crate::pat::PatField;
use crate::*;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatStructBody {
    pub fields: Punctuated<PatField, Comma>,
    pub rest: Option<DotDot>,
}

impl ToTokens for PatStructBody {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.fields.to_tokens(t);
        if let Some(dotdot) = &self.rest {
            dotdot.to_tokens(t);
        }
    }
}

#[doc = "A struct pattern, e.g. `Point { x, y }` or `Point { x, .. }`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatStruct {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub qself: Option<QSelf>,
    pub path: Path,
    pub brace: Delimited<PatStructBody>,
}

impl ToTokens for PatStruct {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.path.to_tokens(t);
        self.brace.to_tokens(t);
    }
}
