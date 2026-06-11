use moxy_token::punct::{Comma, DotDot};
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::pat::PatField;
use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
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

/// A struct pattern, e.g. `Point { x, y }` or `Point { x, .. }`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatStruct {
    pub attrs: Attributes,
    pub qself: Option<QSelf>,
    pub path: Path,
    pub body: Delimited<PatStructBody>,
}

impl Spanner for PatStruct {
    fn span(&self) -> Span {
        let start = if let Some(a) = self.attrs.first() {
            a.span()
        } else if let Some(q) = &self.qself {
            q.span()
        } else {
            self.path.span()
        };
        start.join(self.body.span())
    }
}

impl ToTokens for PatStruct {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.path.to_tokens(t);
        self.body.to_tokens(t);
    }
}

impl PatStruct {
    pub fn into_pattern(self) -> super::Pattern {
        super::Pattern::from(self)
    }
}
