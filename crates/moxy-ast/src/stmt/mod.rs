use moxy_token::Token;
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::{Parse, Span, Spanner, ToTokens, TokenStream};

use crate::Expr;

mod stmt_block;
mod stmt_local;
mod stmt_macro;

pub use stmt_block::*;
pub use stmt_local::*;
pub use stmt_macro::*;

/// A statement in a block.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Stmt {
    Local(Box<StmtLocal>),
    Block(StmtBlock),
    Item(Box<crate::Item>),
    Expr(Box<Expr>, Option<Token![;]>),
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
            Self::Local(v) => v.span(),
            Self::Block(v) => v.span(),
            Self::Item(v) => v.span(),
            Self::Expr(v, semi) => {
                let end = semi.as_ref().map(|s| s.span()).unwrap_or_else(|| v.span());
                v.span().join(end)
            }
            Self::Macro(v) => v.span(),
        }
    }
}

impl Parse for Stmt {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        if let Some(stmt) = stream.parse_if::<StmtLocal>() {
            return Ok(Self::Local(Box::new(stmt)));
        }

        if let Some(stmt) = stream.parse_if::<StmtBlock>() {
            return Ok(Self::Block(stmt));
        }

        if let Some(item) = stream.parse_if::<crate::Item>() {
            return Ok(Self::Item(Box::new(item)));
        }

        if let Some(stmt) = stream.parse_if::<StmtMacro>() {
            return Ok(Self::Macro(stmt));
        }

        let expr = stream.parse::<Expr>()?;
        let semi = stream.parse_if::<Token![;]>();
        Ok(Self::Expr(Box::new(expr), semi))
    }
}

impl ToTokens for Stmt {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            Self::Local(v) => v.to_tokens(t),
            Self::Block(v) => v.to_tokens(t),
            Self::Item(v) => v.to_tokens(t),
            Self::Expr(v, semi) => {
                v.to_tokens(t);
                if let Some(s) = semi {
                    s.to_tokens(t);
                }
            }
            Self::Macro(v) => v.to_tokens(t),
        }
    }
}
