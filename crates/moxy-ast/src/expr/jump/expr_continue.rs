use moxy_token::keyword::Continue;
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A continue expression: `continue`, `continue 'label`.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprContinue {
    pub attrs: Vec<Attribute>,
    pub continue_keyword: Continue,
    pub label: Option<Label>,
}

impl Spanner for ExprContinue {
    fn span(&self) -> Span {
        let start = if let Some(a) = self.attrs.first() {
            a.span()
        } else {
            self.continue_keyword.span()
        };
        let end = if let Some(l) = &self.label {
            l.span()
        } else {
            self.continue_keyword.span()
        };
        start.join(end)
    }
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
