use moxy_token::keyword::Continue;
use moxy_token::{Span, ToTokens, TokenStream};

use crate::*;

#[doc = "A continue expression: `continue`, `continue 'label`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprContinue {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub continue_keyword: Continue,
    pub label: Option<Label>,
}

impl ToTokens for ExprContinue {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.continue_keyword.to_tokens(t);

        if let Some(l) = &self.label {
            l.name.to_tokens(t);
        }
    }
}
