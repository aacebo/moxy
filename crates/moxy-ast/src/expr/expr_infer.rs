use moxy_token::{Ident, Span, Spanner, ToTokenStream, ToTokens, TokenStream, Underscore};

use crate::*;

/// `let _ = ...`
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprInfer {
    pub attrs: Attributes,
    pub underscore: Token![_],
}

impl Attributed for ExprInfer {
    fn attrs(&self) -> &[Attribute] {
        &self.attrs
    }
}

impl From<ExprInfer> for Expr {
    fn from(value: ExprInfer) -> Self {
        Self::Infer(value)
    }
}

impl Spanner for ExprInfer {
    fn span(&self) -> Span {
        self.to_token_stream().span()
    }
}

impl ToTokens for ExprInfer {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        // An identifier, as the lexer and the compiler give `_`
        Ident::new(Underscore::TEXT).with_span(self.underscore.span()).to_tokens(t);
    }
}
