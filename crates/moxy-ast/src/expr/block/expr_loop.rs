use moxy_token::keyword::Loop;
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A loop expression: `loop { ... }`, `'label: loop { ... }`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprLoop {
    pub attrs: Attributes,
    pub label: Option<Label>,
    pub loop_keyword: Loop,
    pub body: StmtBlock,
}

impl Spanner for ExprLoop {
    fn span(&self) -> Span {
        let start = if let Some(a) = self.attrs.first() {
            a.span()
        } else if let Some(l) = &self.label {
            l.span()
        } else {
            self.loop_keyword.span()
        };

        start.join(self.body.span())
    }
}

impl ExprLoop {
    pub fn parse_from(stream: &mut ParseStream, label: Option<Label>) -> Result<Self, ParseError> {
        let loop_keyword = stream.parse::<Loop>()?;
        let body = stream.parse::<StmtBlock>()?;

        Ok(Self {
            attrs: Attributes::default(),
            label,
            loop_keyword,
            body,
        })
    }

    pub fn into_block_expr(self) -> super::BlockExpr {
        super::BlockExpr::from(self)
    }
}

impl ToTokens for ExprLoop {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }

        if let Some(l) = &self.label {
            l.to_tokens(t);
        }

        self.loop_keyword.to_tokens(t);
        self.body.to_tokens(t);
    }
}
