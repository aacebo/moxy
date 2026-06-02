use crate::ast::*;
use crate::token::ToTokens;
use crate::token::keyword::Continue;
use crate::{Span, TokenStream};

#[doc = "A continue expression: `continue`, `continue 'label`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprContinue {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub label: Option<Label>,
}

impl ToTokens for ExprContinue {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        Continue::default().to_tokens(t);

        if let Some(l) = &self.label {
            l.name.to_tokens(t);
        }
    }
}
