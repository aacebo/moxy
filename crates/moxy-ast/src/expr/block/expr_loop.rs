use moxy_token::keyword::Loop;
use moxy_token::parse::{ParseError, ParseStream};
use moxy_token::{Span, ToTokens, TokenStream};

use crate::*;

#[doc = "A loop expression: `loop { ... }`, `'label: loop { ... }`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprLoop {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub label: Option<Label>,
    pub loop_keyword: Loop,
    pub body: StmtBlock,
}

impl ExprLoop {
    pub fn parse_from(stream: &mut ParseStream, label: Option<Label>) -> Result<Self, ParseError> {
        let loop_keyword = stream.parse::<Loop>()?;
        let body = stream.parse::<StmtBlock>()?;
        Ok(Self {
            span: Span::default(),
            attrs: Vec::new(),
            label,
            loop_keyword,
            body,
        })
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
