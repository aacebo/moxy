mod stmt_block;
mod stmt_local;
mod stmt_macro;

pub use stmt_block::*;
pub use stmt_local::*;
pub use stmt_macro::*;

use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A statement in a block.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Stmt {
    Local(Box<StmtLocal>),
    Block(StmtBlock),
    Item(Box<Item>),
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

    pub fn as_item(&self) -> Option<&Item> {
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
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.peek::<StmtLocal>()
            || cursor.peek::<StmtMacro>()
            || cursor.peek::<StmtBlock>()
            || cursor.peek::<Item>()
            || cursor.peek::<Expr>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        if parser.peek::<StmtLocal>() {
            return Ok(Self::Local(Box::new(parser.parse()?)));
        }

        if parser.peek::<StmtMacro>() {
            return Ok(Self::Macro(parser.parse()?));
        }

        if parser.peek::<StmtBlock>() {
            return Ok(Self::Block(parser.parse()?));
        }

        if parser.peek::<Item>() {
            return Ok(Self::Item(parser.parse()?));
        }

        let expr = parser.parse()?;
        let semi = parser.parse()?;
        Ok(Self::Expr(Box::new(expr), semi))
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        if cursor.peek::<StmtLocal>() {
            cursor.skip::<StmtLocal>()
        } else if cursor.peek::<StmtMacro>() {
            cursor.skip::<StmtMacro>()
        } else if cursor.peek::<StmtBlock>() {
            cursor.skip::<StmtBlock>()
        } else if cursor.peek::<Item>() {
            cursor.skip::<Item>()
        } else {
            cursor.skip::<Expr>()?.skip::<Option<Token![;]>>()
        }
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
                semi.to_tokens(t);
            }
            Self::Macro(v) => v.to_tokens(t),
        }
    }
}
