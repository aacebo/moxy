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

impl Stmt {
    pub fn is_local(&self) -> bool {
        matches!(self, Self::Local(_))
    }

    pub fn is_block(&self) -> bool {
        matches!(self, Self::Block(_))
    }

    pub fn is_item(&self) -> bool {
        matches!(self, Self::Item(_))
    }

    pub fn is_expr(&self) -> bool {
        matches!(self, Self::Expr(..))
    }

    pub fn is_macro(&self) -> bool {
        matches!(self, Self::Macro(_))
    }

    pub fn as_local(&self) -> Option<&StmtLocal> {
        if let Self::Local(v) = self { Some(v.as_ref()) } else { None }
    }

    pub fn as_block(&self) -> Option<&StmtBlock> {
        if let Self::Block(v) = self { Some(v) } else { None }
    }

    pub fn as_item(&self) -> Option<&crate::Item> {
        if let Self::Item(v) = self { Some(v.as_ref()) } else { None }
    }

    pub fn as_macro(&self) -> Option<&StmtMacro> {
        if let Self::Macro(v) = self { Some(v) } else { None }
    }
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
