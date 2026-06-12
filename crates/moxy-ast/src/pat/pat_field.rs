use moxy_token::punct::Colon;
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A single field binding inside a struct pattern, e.g. `x` (shorthand) or `x: pat`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatField {
    pub attrs: Attributes,
    pub member: Member,
    pub colon: Option<Colon>,
    pub pat: Pattern,
    pub shorthand: bool,
}

impl Spanner for PatField {
    fn span(&self) -> Span {
        self.attrs.span().join(self.pat.span())
    }
}

impl ToTokens for PatField {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);

        if self.shorthand {
            self.pat.to_tokens(t);
        } else {
            self.member.to_tokens(t);
            self.colon.to_tokens(t);
            self.pat.to_tokens(t);
        }
    }
}
