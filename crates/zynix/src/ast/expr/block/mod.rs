mod expr_async;
mod expr_brace;
mod expr_const;
mod expr_for_loop;
mod expr_if;
mod expr_loop;
mod expr_match;
mod expr_try_block;
mod expr_unsafe;
mod expr_while;

pub use expr_async::*;
pub use expr_brace::*;
pub use expr_const::*;
pub use expr_for_loop::*;
pub use expr_if::*;
pub use expr_loop::*;
pub use expr_match::*;
pub use expr_try_block::*;
pub use expr_unsafe::*;
pub use expr_while::*;

use super::Expr;
use crate::Span;
use crate::ast::{Label, Lifetime, Pattern, StmtBlock};
use crate::parse::{ParseError, ParseStream};
use crate::token::keyword::{For, If, In, Loop, Match, While};
use crate::token::{Delim, Punctuation, ToTokens, Token, TokenStream, TokenTree};

#[doc = "Block-like expressions (braced blocks, if, while, for, loop, match, async, unsafe, const, try)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum BlockExpr {
    Brace(ExprBrace),
    If(ExprIf),
    While(ExprWhile),
    ForLoop(ExprForLoop),
    Loop(ExprLoop),
    Match(ExprMatch),
    Async(ExprAsync),
    Unsafe(ExprUnsafe),
    Const(ExprConst),
    TryBlock(ExprTryBlock),
}

impl ToTokens for BlockExpr {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            BlockExpr::Brace(v) => v.to_tokens(t),
            BlockExpr::If(v) => v.to_tokens(t),
            BlockExpr::While(v) => v.to_tokens(t),
            BlockExpr::ForLoop(v) => v.to_tokens(t),
            BlockExpr::Loop(v) => v.to_tokens(t),
            BlockExpr::Match(v) => v.to_tokens(t),
            BlockExpr::Async(v) => v.to_tokens(t),
            BlockExpr::Unsafe(v) => v.to_tokens(t),
            BlockExpr::Const(v) => v.to_tokens(t),
            BlockExpr::TryBlock(v) => v.to_tokens(t),
        }
    }
}

impl From<ExprBrace> for BlockExpr {
    fn from(v: ExprBrace) -> Self {
        BlockExpr::Brace(v)
    }
}

impl From<ExprIf> for BlockExpr {
    fn from(v: ExprIf) -> Self {
        BlockExpr::If(v)
    }
}

impl From<ExprWhile> for BlockExpr {
    fn from(v: ExprWhile) -> Self {
        BlockExpr::While(v)
    }
}

impl From<ExprForLoop> for BlockExpr {
    fn from(v: ExprForLoop) -> Self {
        BlockExpr::ForLoop(v)
    }
}

impl From<ExprLoop> for BlockExpr {
    fn from(v: ExprLoop) -> Self {
        BlockExpr::Loop(v)
    }
}

impl From<ExprMatch> for BlockExpr {
    fn from(v: ExprMatch) -> Self {
        BlockExpr::Match(v)
    }
}

impl From<ExprAsync> for BlockExpr {
    fn from(v: ExprAsync) -> Self {
        BlockExpr::Async(v)
    }
}

impl From<ExprUnsafe> for BlockExpr {
    fn from(v: ExprUnsafe) -> Self {
        BlockExpr::Unsafe(v)
    }
}

impl From<ExprConst> for BlockExpr {
    fn from(v: ExprConst) -> Self {
        BlockExpr::Const(v)
    }
}

impl From<ExprTryBlock> for BlockExpr {
    fn from(v: ExprTryBlock) -> Self {
        BlockExpr::TryBlock(v)
    }
}

// Parser helpers

impl ExprIf {
    pub fn parse_from(stream: &mut ParseStream) -> Result<Expr, ParseError> {
        let _ = stream.parse::<If>()?;
        let cond = Box::new(super::parse_expr(stream, false)?);
        let then_branch = stream.parse::<StmtBlock>()?;

        let else_branch = if matches!(stream.curr(), Some(tt) if tt.name().as_deref() == Some("else")) {
            stream.advance();
            Some(Box::new(super::primary::PrimaryExpr::parse_from(stream, true)?))
        } else {
            None
        };

        Ok(Expr::Block(BlockExpr::If(ExprIf {
            span: Span::default(),
            attrs: Vec::new(),
            cond,
            then_branch,
            else_branch,
        })))
    }
}

impl ExprWhile {
    pub fn parse_from(stream: &mut ParseStream, label: Option<Label>) -> Result<Self, ParseError> {
        let _ = stream.parse::<While>()?;
        let cond = Box::new(super::parse_expr(stream, false)?);
        let body = stream.parse::<StmtBlock>()?;
        Ok(Self {
            span: Span::default(),
            attrs: Vec::new(),
            label,
            cond,
            body,
        })
    }
}

impl ExprForLoop {
    pub fn parse_from(stream: &mut ParseStream, label: Option<Label>) -> Result<Self, ParseError> {
        let _ = stream.parse::<For>()?;
        let pat = Box::new(stream.parse::<Pattern>()?);
        let _ = stream.parse::<In>()?;
        let expr = Box::new(super::parse_expr(stream, false)?);
        let body = stream.parse::<StmtBlock>()?;
        Ok(Self {
            span: Span::default(),
            attrs: Vec::new(),
            label,
            pat,
            expr,
            body,
        })
    }
}

impl ExprLoop {
    pub fn parse_from(stream: &mut ParseStream, label: Option<Label>) -> Result<Self, ParseError> {
        let _ = stream.parse::<Loop>()?;
        let body = stream.parse::<StmtBlock>()?;
        Ok(Self {
            span: Span::default(),
            attrs: Vec::new(),
            label,
            body,
        })
    }
}

impl ExprMatch {
    pub fn parse_from(stream: &mut ParseStream) -> Result<Expr, ParseError> {
        let _ = stream.parse::<Match>()?;
        let expr = Box::new(super::parse_expr(stream, false)?);
        let group = stream.parse_group(Delim::Brace)?;
        let mut inner = group.parse();
        let arms = inner.parse_vec::<crate::ast::MatchArm>()?;
        Ok(Expr::Block(BlockExpr::Match(ExprMatch {
            span: Span::default(),
            attrs: Vec::new(),
            expr,
            arms,
        })))
    }
}

impl Label {
    /// Parse a bare lifetime label (no trailing `:`) for `break`/`continue`.
    pub fn parse_opt_break(stream: &mut ParseStream) -> Option<Self> {
        if !matches!(stream.curr(), Some(TokenTree::Token(Token::Punct(Punctuation::Quote(_))))) {
            return None;
        }

        let name = stream.parse_opt::<Lifetime>()?;
        Some(Label {
            span: Span::default(),
            name,
        })
    }
}
