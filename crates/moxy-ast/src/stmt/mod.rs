use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::Semi;
use moxy_token::{Parse, ToTokens, TokenStream};

use crate::Expr;

mod stmt_block;
mod stmt_local;
mod stmt_macro;

pub use stmt_block::*;
pub use stmt_local::*;
pub use stmt_macro::*;

#[doc = "A statement in a block."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Stmt {
    Local(Box<StmtLocal>),
    Block(StmtBlock),
    Item(Box<crate::Item>),
    Expr(Box<Expr>, Option<Semi>),
    Macro(StmtMacro),
}

impl Parse for Stmt {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        if stream.peek::<StmtLocal>().is_some() {
            return Ok(Stmt::Local(Box::new(stream.parse()?)));
        }
        if stream.peek::<StmtBlock>().is_some() {
            return Ok(Stmt::Block(stream.parse()?));
        }
        if stream.peek::<crate::Item>().is_some() {
            return Ok(Stmt::Item(Box::new(stream.parse()?)));
        }
        if stream.peek::<StmtMacro>().is_some() {
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
