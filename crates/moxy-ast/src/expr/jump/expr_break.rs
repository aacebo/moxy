use moxy_token::keyword::Break;
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A break expression: `break`, `break 'label`, `break expr`.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprBreak {
    pub attrs: Vec<Attribute>,
    pub break_keyword: Break,
    pub label: Option<Label>,
    pub expr: Option<Box<Expr>>,
}

impl Spanner for ExprBreak {
    fn span(&self) -> Span {
        let start = if let Some(a) = self.attrs.first() {
            a.span()
        } else {
            self.break_keyword.span()
        };
        let end = if let Some(e) = &self.expr {
            e.span()
        } else if let Some(l) = &self.label {
            l.span()
        } else {
            self.break_keyword.span()
        };
        start.join(end)
    }
}

impl ToTokens for ExprBreak {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.break_keyword.to_tokens(t);

        if let Some(l) = &self.label {
            l.name.to_tokens(t);
        }

        if let Some(e) = &self.expr {
            e.to_tokens(t);
        }
    }
}

impl ExprBreak {
    pub fn into_jump_expr(self) -> super::JumpExpr {
        super::JumpExpr::from(self)
    }
}
