use moxy_token::punct::Colon;
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A type-ascription pattern, e.g. `x: i32`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatType {
    pub attrs: Attributes,
    pub pat: Box<Pattern>,
    pub colon: Colon,
    pub ty: Box<Type>,
}

impl Spanner for PatType {
    fn span(&self) -> Span {
        let start = if let Some(a) = self.attrs.first() {
            a.span()
        } else {
            self.pat.span()
        };
        start.join(self.ty.span())
    }
}

impl ToTokens for PatType {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.pat.to_tokens(t);
        self.colon.to_tokens(t);
        self.ty.to_tokens(t);
    }
}

impl PatType {
    pub fn into_pattern(self) -> super::Pattern {
        super::Pattern::from(self)
    }
}
