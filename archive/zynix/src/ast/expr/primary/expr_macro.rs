use crate::ast::{Attribute, Expr, MacroCall};
use crate::parse::{ParseError, ParseStream};
use crate::token::{LexError, ToTokens};
use crate::{Parse, Span, TokenStream};

#[doc = "A macro invocation expression (`path!(...)`, `path![...]`, `path!{...}`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprMacro {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub mac: MacroCall,
}

impl Parse for ExprMacro {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        use crate::ast::PrimaryExpr;
        let at = stream.span();
        match Expr::parse(stream)? {
            Expr::Primary(PrimaryExpr::Macro(v)) => Ok(v),
            _ => Err(LexError::new(at).message("expected macro expression").into()),
        }
    }
}

impl ToTokens for ExprMacro {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.mac.to_tokens(t);
    }
}
