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
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::{Punctuation, Span, Spanner, ToTokens, Token, TokenStream, TokenTree};

use crate::{Label, Lifetime};

/// Block-like expressions (braced blocks, if, while, for, loop, match, async, unsafe, const, try).
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

impl Spanner for BlockExpr {
    fn span(&self) -> Span {
        match self {
            BlockExpr::Brace(v) => v.span(),
            BlockExpr::If(v) => v.span(),
            BlockExpr::While(v) => v.span(),
            BlockExpr::ForLoop(v) => v.span(),
            BlockExpr::Loop(v) => v.span(),
            BlockExpr::Match(v) => v.span(),
            BlockExpr::Async(v) => v.span(),
            BlockExpr::Unsafe(v) => v.span(),
            BlockExpr::Const(v) => v.span(),
            BlockExpr::TryBlock(v) => v.span(),
        }
    }
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

impl Label {
    pub fn parse_opt_break(stream: &mut ParseStream) -> Option<Self> {
        if !matches!(stream.curr(), Some(TokenTree::Token(Token::Punct(Punctuation::Quote(_))))) {
            return None;
        }

        let name = stream.parse_if::<Lifetime>()?;
        let colon = stream.parse_if::<moxy_token::punct::Colon>().unwrap_or_default();
        Some(Label { name, colon })
    }
}
