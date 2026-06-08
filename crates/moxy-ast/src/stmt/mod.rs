use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::Semi;
use moxy_token::{Parse, Span, Spanner, ToTokens, TokenStream};

use crate::Expr;

mod stmt_block;
mod stmt_local;
mod stmt_macro;

pub use stmt_block::*;
pub use stmt_local::*;
pub use stmt_macro::*;

/// A statement in a block.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Stmt {
    Local(Box<StmtLocal>),
    Block(StmtBlock),
    Item(Box<crate::Item>),
    Expr(Box<Expr>, Option<Semi>),
    Macro(StmtMacro),
}

impl Spanner for Stmt {
    fn span(&self) -> Span {
        match self {
            Stmt::Local(v) => v.span(),
            Stmt::Block(v) => v.span(),
            Stmt::Item(v) => v.span(),
            Stmt::Expr(v, semi) => {
                let end = semi.as_ref().map(|s| s.span()).unwrap_or_else(|| v.span());
                v.span().join(end)
            }
            Stmt::Macro(v) => v.span(),
        }
    }
}

impl Parse for Stmt {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        if stream.peek::<StmtLocal>() {
            return Ok(Stmt::Local(Box::new(stream.parse()?)));
        }
        if stream.peek::<StmtBlock>() {
            return Ok(Stmt::Block(stream.parse()?));
        }
        if stream.peek::<crate::Item>() {
            return Ok(Stmt::Item(Box::new(stream.parse()?)));
        }
        if stream.peek::<StmtMacro>() {
            return Ok(Stmt::Macro(stream.parse()?));
        }
        let expr = stream.parse::<Expr>()?;
        let semi = stream.parse_if::<Semi>();
        Ok(Stmt::Expr(Box::new(expr), semi))
    }
}

impl ToTokens for Stmt {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            Stmt::Local(v) => v.to_tokens(t),
            Stmt::Block(v) => v.to_tokens(t),
            Stmt::Item(v) => v.to_tokens(t),
            Stmt::Expr(v, semi) => {
                v.to_tokens(t);
                if let Some(s) = semi {
                    s.to_tokens(t);
                }
            }
            Stmt::Macro(v) => v.to_tokens(t),
        }
    }
}
