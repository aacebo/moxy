use crate::Token;
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A continue expression: `continue`, `continue 'label`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprContinue {
    pub attrs: Attributes,
    pub continue_keyword: Token![continue],
    pub label: Option<Label>,
}

impl Spanner for ExprContinue {
    fn span(&self) -> Span {
        let end = if let Some(l) = &self.label {
            l.span()
        } else {
            self.continue_keyword.span()
        };
        self.attrs.span().join(end)
    }
}

impl ToTokens for ExprContinue {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.continue_keyword.to_tokens(t);

        if let Some(l) = &self.label {
            l.name.to_tokens(t);
        }
    }
}

impl ExprContinue {
    pub fn into_jump_expr(self) -> super::JumpExpr {
        super::JumpExpr::from(self)
    }
}
