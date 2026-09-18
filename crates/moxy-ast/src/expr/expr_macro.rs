use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A macro invocation expression (`path!(...)`, `path![...]`, `path!{...}`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprMacro {
    pub attrs: Attributes,
    pub mac: MacroCall,
}

impl From<ExprMacro> for Expr {
    fn from(value: ExprMacro) -> Self {
        Self::Macro(value)
    }
}

impl Spanner for ExprMacro {
    fn span(&self) -> Span {
        self.attrs.span().join(self.mac.span())
    }
}

impl ToTokens for ExprMacro {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.mac.to_tokens(t);
    }
}
