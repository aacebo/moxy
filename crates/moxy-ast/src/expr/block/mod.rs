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
use moxy_token::parser::ParseStream;
use moxy_token::{Punctuation, Span, Spanner, ToTokens, TokenStream, TokenTree};

use crate::{Attributes, Label, Lifetime};

/// Block-like expressions (braced blocks, if, while, for, loop, match, async, unsafe, const, try).
#[derive(Debug, Clone, PartialEq, Eq)]
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

impl BlockExpr {
    pub fn attrs(&self) -> &Attributes {
        match self {
            Self::Brace(v) => &v.attrs,
            Self::If(v) => &v.attrs,
            Self::While(v) => &v.attrs,
            Self::ForLoop(v) => &v.attrs,
            Self::Loop(v) => &v.attrs,
            Self::Match(v) => &v.attrs,
            Self::Async(v) => &v.attrs,
            Self::Unsafe(v) => &v.attrs,
            Self::Const(v) => &v.attrs,
            Self::TryBlock(v) => &v.attrs,
        }
    }

    pub fn attrs_mut(&mut self) -> &mut Attributes {
        match self {
            Self::Brace(v) => &mut v.attrs,
            Self::If(v) => &mut v.attrs,
            Self::While(v) => &mut v.attrs,
            Self::ForLoop(v) => &mut v.attrs,
            Self::Loop(v) => &mut v.attrs,
            Self::Match(v) => &mut v.attrs,
            Self::Async(v) => &mut v.attrs,
            Self::Unsafe(v) => &mut v.attrs,
            Self::Const(v) => &mut v.attrs,
            Self::TryBlock(v) => &mut v.attrs,
        }
    }

    pub fn is_brace(&self) -> bool {
        matches!(self, Self::Brace(_))
    }

    pub fn is_if(&self) -> bool {
        matches!(self, Self::If(_))
    }

    pub fn is_while(&self) -> bool {
        matches!(self, Self::While(_))
    }

    pub fn is_for_loop(&self) -> bool {
        matches!(self, Self::ForLoop(_))
    }

    pub fn is_loop(&self) -> bool {
        matches!(self, Self::Loop(_))
    }

    pub fn is_match(&self) -> bool {
        matches!(self, Self::Match(_))
    }

    pub fn is_async(&self) -> bool {
        matches!(self, Self::Async(_))
    }

    pub fn is_unsafe(&self) -> bool {
        matches!(self, Self::Unsafe(_))
    }

    pub fn is_const(&self) -> bool {
        matches!(self, Self::Const(_))
    }

    pub fn is_try_block(&self) -> bool {
        matches!(self, Self::TryBlock(_))
    }

    pub fn as_brace(&self) -> Option<&ExprBrace> {
        if let Self::Brace(v) = self { Some(v) } else { None }
    }

    pub fn as_if(&self) -> Option<&ExprIf> {
        if let Self::If(v) = self { Some(v) } else { None }
    }

    pub fn as_while(&self) -> Option<&ExprWhile> {
        if let Self::While(v) = self { Some(v) } else { None }
    }

    pub fn as_for_loop(&self) -> Option<&ExprForLoop> {
        if let Self::ForLoop(v) = self { Some(v) } else { None }
    }

    pub fn as_loop(&self) -> Option<&ExprLoop> {
        if let Self::Loop(v) = self { Some(v) } else { None }
    }

    pub fn as_match(&self) -> Option<&ExprMatch> {
        if let Self::Match(v) = self { Some(v) } else { None }
    }

    pub fn as_async(&self) -> Option<&ExprAsync> {
        if let Self::Async(v) = self { Some(v) } else { None }
    }

    pub fn as_unsafe(&self) -> Option<&ExprUnsafe> {
        if let Self::Unsafe(v) = self { Some(v) } else { None }
    }

    pub fn as_const(&self) -> Option<&ExprConst> {
        if let Self::Const(v) = self { Some(v) } else { None }
    }

    pub fn as_try_block(&self) -> Option<&ExprTryBlock> {
        if let Self::TryBlock(v) = self { Some(v) } else { None }
    }

    pub fn into_expr(self) -> super::Expr {
        super::Expr::from(self)
    }
}

impl Spanner for BlockExpr {
    fn span(&self) -> Span {
        match self {
            Self::Brace(v) => v.span(),
            Self::If(v) => v.span(),
            Self::While(v) => v.span(),
            Self::ForLoop(v) => v.span(),
            Self::Loop(v) => v.span(),
            Self::Match(v) => v.span(),
            Self::Async(v) => v.span(),
            Self::Unsafe(v) => v.span(),
            Self::Const(v) => v.span(),
            Self::TryBlock(v) => v.span(),
        }
    }
}

impl ToTokens for BlockExpr {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            Self::Brace(v) => v.to_tokens(t),
            Self::If(v) => v.to_tokens(t),
            Self::While(v) => v.to_tokens(t),
            Self::ForLoop(v) => v.to_tokens(t),
            Self::Loop(v) => v.to_tokens(t),
            Self::Match(v) => v.to_tokens(t),
            Self::Async(v) => v.to_tokens(t),
            Self::Unsafe(v) => v.to_tokens(t),
            Self::Const(v) => v.to_tokens(t),
            Self::TryBlock(v) => v.to_tokens(t),
        }
    }
}

impl From<ExprBrace> for BlockExpr {
    fn from(v: ExprBrace) -> Self {
        Self::Brace(v)
    }
}

impl From<ExprIf> for BlockExpr {
    fn from(v: ExprIf) -> Self {
        Self::If(v)
    }
}

impl From<ExprWhile> for BlockExpr {
    fn from(v: ExprWhile) -> Self {
        Self::While(v)
    }
}

impl From<ExprForLoop> for BlockExpr {
    fn from(v: ExprForLoop) -> Self {
        Self::ForLoop(v)
    }
}

impl From<ExprLoop> for BlockExpr {
    fn from(v: ExprLoop) -> Self {
        Self::Loop(v)
    }
}

impl From<ExprMatch> for BlockExpr {
    fn from(v: ExprMatch) -> Self {
        Self::Match(v)
    }
}

impl From<ExprAsync> for BlockExpr {
    fn from(v: ExprAsync) -> Self {
        Self::Async(v)
    }
}

impl From<ExprUnsafe> for BlockExpr {
    fn from(v: ExprUnsafe) -> Self {
        Self::Unsafe(v)
    }
}

impl From<ExprConst> for BlockExpr {
    fn from(v: ExprConst) -> Self {
        Self::Const(v)
    }
}

impl From<ExprTryBlock> for BlockExpr {
    fn from(v: ExprTryBlock) -> Self {
        Self::TryBlock(v)
    }
}

impl Label {
    pub fn parse_opt_break(stream: &mut ParseStream) -> Option<Self> {
        if !matches!(stream.curr(), Some(TokenTree::Punct(Punctuation::Quote(_)))) {
            return None;
        }

        let name = stream.parse_if::<Lifetime>()?;
        let colon = stream.parse_if::<moxy_token::punct::Colon>().unwrap_or_default();
        Some(Self { name, colon })
    }
}
