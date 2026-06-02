use crate::ast::*;
use crate::token::ToTokens;
use crate::token::punct::Colon;
use crate::{Span, TokenStream};

#[doc = "A single field binding inside a struct pattern, e.g. `x` (shorthand) or `x: pat`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatField {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub member: Member,
    pub pat: Pattern,
    pub shorthand: bool,
}

impl ToTokens for PatField {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        if self.shorthand {
            self.pat.to_tokens(t);
        } else {
            self.member.to_tokens(t);
            Colon::default().to_tokens(t);
            self.pat.to_tokens(t);
        }
    }
}
