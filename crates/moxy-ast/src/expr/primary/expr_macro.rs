use crate::{Parse, ParseError, Parser};
use moxy_token::{LexError, Span, Spanner, ToTokens, TokenStream};

use crate::{Attributes, Expr, MacroCall};

/// A macro invocation expression (`path!(...)`, `path![...]`, `path!{...}`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprMacro {
    pub attrs: Attributes,
    pub mac: MacroCall,
}

impl Spanner for ExprMacro {
    fn span(&self) -> Span {
        self.attrs.span().join(self.mac.span())
    }
}

impl Parse for ExprMacro {
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        use crate::PrimaryExpr;
        let at = parser.span();
        match parser.parse::<Expr>()? {
            Expr::Primary(PrimaryExpr::Macro(v)) => Ok(v),
            _ => Err(LexError::new(at).message("expected macro expression").into()),
        }
    }
}

impl ToTokens for ExprMacro {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.mac.to_tokens(t);
    }
}

impl ExprMacro {
    pub fn into_primary_expr(self) -> super::PrimaryExpr {
        super::PrimaryExpr::from(self)
    }
}
