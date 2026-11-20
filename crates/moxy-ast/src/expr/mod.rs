mod expr_array;
mod expr_assign;
mod expr_async;
mod expr_binary_chain;
mod expr_block;
mod expr_break;
mod expr_cast_chain;
mod expr_closure;
mod expr_const;
mod expr_continue;
mod expr_for_loop;
mod expr_group;
mod expr_if_chain;
mod expr_infer;
mod expr_let;
mod expr_lit;
mod expr_loop;
mod expr_macro;
mod expr_match;
mod expr_paren;
mod expr_path;
mod expr_postfix;
mod expr_range;
mod expr_raw_addr;
mod expr_reference;
mod expr_repeat;
mod expr_return;
mod expr_struct;
mod expr_try_block;
mod expr_tuple;
mod expr_unary;
mod expr_unsafe;
mod expr_while;
mod expr_yield;
mod postfix_await;
mod postfix_call;
mod postfix_field;
mod postfix_index;
mod postfix_method_call;
mod postfix_try;

pub(crate) mod parse;
pub(crate) mod peek;
pub(crate) mod skip;

pub use expr_array::*;
pub use expr_assign::*;
pub use expr_async::*;
pub use expr_binary_chain::*;
pub use expr_block::*;
pub use expr_break::*;
pub use expr_cast_chain::*;
pub use expr_closure::*;
pub use expr_const::*;
pub use expr_continue::*;
pub use expr_for_loop::*;
pub use expr_group::*;
pub use expr_if_chain::*;
pub use expr_infer::*;
pub use expr_let::*;
pub use expr_lit::*;
pub use expr_loop::*;
pub use expr_macro::*;
pub use expr_match::*;
pub use expr_paren::*;
pub use expr_path::*;
pub use expr_postfix::*;
pub use expr_range::*;
pub use expr_raw_addr::*;
pub use expr_reference::*;
pub use expr_repeat::*;
pub use expr_return::*;
pub use expr_struct::*;
pub use expr_try_block::*;
pub use expr_tuple::*;
pub use expr_unary::*;
pub use expr_unsafe::*;
pub use expr_while::*;
pub use expr_yield::*;
pub use postfix_await::*;
pub use postfix_call::*;
pub use postfix_field::*;
pub use postfix_index::*;
pub use postfix_method_call::*;
pub use postfix_try::*;

use moxy_token::{Delim, Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A Rust expression. The primary recursive node covering all expression forms.
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[non_exhaustive]
pub enum Expr {
    Array(ExprArray),
    Assign(ExprAssign),
    Async(ExprAsync),
    BinaryChain(ExprBinaryChain),
    Block(ExprBlock),
    Break(ExprBreak),
    CastChain(ExprCastChain),
    Closure(ExprClosure),
    Const(ExprConst),
    Continue(ExprContinue),
    ForLoop(ExprForLoop),
    Group(ExprGroup),
    IfChain(ExprIfChain),
    Infer(ExprInfer),
    Let(ExprLet),
    Lit(ExprLit),
    Loop(ExprLoop),
    Macro(ExprMacro),
    Match(ExprMatch),
    Paren(ExprParen),
    Path(ExprPath),
    Postfix(ExprPostfix),
    Range(ExprRange),
    RawAddr(ExprRawAddr),
    Reference(ExprReference),
    Repeat(ExprRepeat),
    Return(ExprReturn),
    Struct(ExprStruct),
    TryBlock(ExprTryBlock),
    Tuple(ExprTuple),
    Unary(ExprUnary),
    Unsafe(ExprUnsafe),
    Verbatim(TokenStream),
    While(ExprWhile),
    Yield(ExprYield),
}

clone_in_small_frames!(Expr {
    Array,
    Assign,
    Async,
    BinaryChain,
    Block,
    Break,
    CastChain,
    Closure,
    Const,
    Continue,
    ForLoop,
    Group,
    IfChain,
    Infer,
    Let,
    Lit,
    Loop,
    Macro,
    Match,
    Paren,
    Path,
    Postfix,
    Range,
    RawAddr,
    Reference,
    Repeat,
    Return,
    Struct,
    TryBlock,
    Tuple,
    Unary,
    Unsafe,
    Verbatim,
    While,
    Yield
});

impl Expr {
    pub fn attrs(&self) -> Option<&Attributes> {
        match self {
            Self::Array(v) => Some(&v.attrs),
            Self::Assign(v) => Some(&v.attrs),
            Self::Async(v) => Some(&v.attrs),
            Self::BinaryChain(v) => Some(&v.attrs),
            Self::Block(v) => Some(&v.attrs),
            Self::Break(v) => Some(&v.attrs),
            Self::CastChain(v) => Some(&v.attrs),
            Self::Closure(v) => Some(&v.attrs),
            Self::Const(v) => Some(&v.attrs),
            Self::Continue(v) => Some(&v.attrs),
            Self::ForLoop(v) => Some(&v.attrs),
            Self::Group(v) => Some(&v.attrs),
            Self::IfChain(v) => Some(&v.attrs),
            Self::Infer(v) => Some(&v.attrs),
            Self::Let(v) => Some(&v.attrs),
            Self::Lit(v) => Some(&v.attrs),
            Self::Loop(v) => Some(&v.attrs),
            Self::Macro(v) => Some(&v.attrs),
            Self::Match(v) => Some(&v.attrs),
            Self::Paren(v) => Some(&v.attrs),
            Self::Path(v) => Some(&v.attrs),
            Self::Postfix(v) => Some(&v.attrs),
            Self::Range(v) => Some(&v.attrs),
            Self::RawAddr(v) => Some(&v.attrs),
            Self::Reference(v) => Some(&v.attrs),
            Self::Repeat(v) => Some(&v.attrs),
            Self::Return(v) => Some(&v.attrs),
            Self::Struct(v) => Some(&v.attrs),
            Self::TryBlock(v) => Some(&v.attrs),
            Self::Tuple(v) => Some(&v.attrs),
            Self::Unary(v) => Some(&v.attrs),
            Self::Unsafe(v) => Some(&v.attrs),
            Self::While(v) => Some(&v.attrs),
            Self::Yield(v) => Some(&v.attrs),
            _ => None,
        }
    }

    pub fn attrs_mut(&mut self) -> Option<&mut Attributes> {
        match self {
            Self::Array(v) => Some(&mut v.attrs),
            Self::Assign(v) => Some(&mut v.attrs),
            Self::Async(v) => Some(&mut v.attrs),
            Self::BinaryChain(v) => Some(&mut v.attrs),
            Self::Block(v) => Some(&mut v.attrs),
            Self::Break(v) => Some(&mut v.attrs),
            Self::CastChain(v) => Some(&mut v.attrs),
            Self::Closure(v) => Some(&mut v.attrs),
            Self::Const(v) => Some(&mut v.attrs),
            Self::Continue(v) => Some(&mut v.attrs),
            Self::ForLoop(v) => Some(&mut v.attrs),
            Self::Group(v) => Some(&mut v.attrs),
            Self::IfChain(v) => Some(&mut v.attrs),
            Self::Infer(v) => Some(&mut v.attrs),
            Self::Let(v) => Some(&mut v.attrs),
            Self::Lit(v) => Some(&mut v.attrs),
            Self::Loop(v) => Some(&mut v.attrs),
            Self::Macro(v) => Some(&mut v.attrs),
            Self::Match(v) => Some(&mut v.attrs),
            Self::Paren(v) => Some(&mut v.attrs),
            Self::Path(v) => Some(&mut v.attrs),
            Self::Postfix(v) => Some(&mut v.attrs),
            Self::Range(v) => Some(&mut v.attrs),
            Self::RawAddr(v) => Some(&mut v.attrs),
            Self::Reference(v) => Some(&mut v.attrs),
            Self::Repeat(v) => Some(&mut v.attrs),
            Self::Return(v) => Some(&mut v.attrs),
            Self::Struct(v) => Some(&mut v.attrs),
            Self::TryBlock(v) => Some(&mut v.attrs),
            Self::Tuple(v) => Some(&mut v.attrs),
            Self::Unary(v) => Some(&mut v.attrs),
            Self::Unsafe(v) => Some(&mut v.attrs),
            Self::While(v) => Some(&mut v.attrs),
            Self::Yield(v) => Some(&mut v.attrs),
            _ => None,
        }
    }

    pub fn is_array(&self) -> bool {
        matches!(self, Self::Array(_))
    }

    pub fn is_assign(&self) -> bool {
        matches!(self, Self::Assign(_))
    }

    pub fn is_async(&self) -> bool {
        matches!(self, Self::Async(_))
    }

    pub fn is_binary_chain(&self) -> bool {
        matches!(self, Self::BinaryChain(_))
    }

    pub fn is_block(&self) -> bool {
        matches!(self, Self::Block(_))
    }

    pub fn is_break(&self) -> bool {
        matches!(self, Self::Break(_))
    }

    pub fn is_cast_chain(&self) -> bool {
        matches!(self, Self::CastChain(_))
    }

    pub fn is_closure(&self) -> bool {
        matches!(self, Self::Closure(_))
    }

    pub fn is_const(&self) -> bool {
        matches!(self, Self::Const(_))
    }

    pub fn is_continue(&self) -> bool {
        matches!(self, Self::Continue(_))
    }

    pub fn is_for_loop(&self) -> bool {
        matches!(self, Self::ForLoop(_))
    }

    pub fn is_group(&self) -> bool {
        matches!(self, Self::Group(_))
    }

    pub fn is_if_chain(&self) -> bool {
        matches!(self, Self::IfChain(_))
    }

    pub fn is_infer(&self) -> bool {
        matches!(self, Self::Infer(_))
    }

    pub fn is_let(&self) -> bool {
        matches!(self, Self::Let(_))
    }

    pub fn is_lit(&self) -> bool {
        matches!(self, Self::Lit(_))
    }

    pub fn is_loop(&self) -> bool {
        matches!(self, Self::Loop(_))
    }

    pub fn is_macro(&self) -> bool {
        matches!(self, Self::Macro(_))
    }

    pub fn is_match(&self) -> bool {
        matches!(self, Self::Match(_))
    }

    pub fn is_paren(&self) -> bool {
        matches!(self, Self::Paren(_))
    }

    pub fn is_path(&self) -> bool {
        matches!(self, Self::Path(_))
    }

    pub fn is_postfix(&self) -> bool {
        matches!(self, Self::Postfix(_))
    }

    pub fn is_range(&self) -> bool {
        matches!(self, Self::Range(_))
    }

    pub fn is_raw_addr(&self) -> bool {
        matches!(self, Self::RawAddr(_))
    }

    pub fn is_reference(&self) -> bool {
        matches!(self, Self::Reference(_))
    }

    pub fn is_repeat(&self) -> bool {
        matches!(self, Self::Repeat(_))
    }

    pub fn is_return(&self) -> bool {
        matches!(self, Self::Return(_))
    }

    pub fn is_struct(&self) -> bool {
        matches!(self, Self::Struct(_))
    }

    pub fn is_try_block(&self) -> bool {
        matches!(self, Self::TryBlock(_))
    }

    pub fn is_tuple(&self) -> bool {
        matches!(self, Self::Tuple(_))
    }

    pub fn is_unary(&self) -> bool {
        matches!(self, Self::Unary(_))
    }

    pub fn is_unsafe(&self) -> bool {
        matches!(self, Self::Unsafe(_))
    }

    pub fn is_verbatim(&self) -> bool {
        matches!(self, Self::Verbatim(_))
    }

    pub fn is_while(&self) -> bool {
        matches!(self, Self::While(_))
    }

    pub fn is_yield(&self) -> bool {
        matches!(self, Self::Yield(_))
    }

    pub fn as_array(&self) -> Option<&ExprArray> {
        if let Self::Array(v) = self { Some(v) } else { None }
    }

    pub fn as_assign(&self) -> Option<&ExprAssign> {
        if let Self::Assign(v) = self { Some(v) } else { None }
    }

    pub fn as_async(&self) -> Option<&ExprAsync> {
        if let Self::Async(v) = self { Some(v) } else { None }
    }

    pub fn as_binary_chain(&self) -> Option<&ExprBinaryChain> {
        if let Self::BinaryChain(v) = self { Some(v) } else { None }
    }

    pub fn as_block(&self) -> Option<&ExprBlock> {
        if let Self::Block(v) = self { Some(v) } else { None }
    }

    pub fn as_break(&self) -> Option<&ExprBreak> {
        if let Self::Break(v) = self { Some(v) } else { None }
    }

    pub fn as_cast_chain(&self) -> Option<&ExprCastChain> {
        if let Self::CastChain(v) = self { Some(v) } else { None }
    }

    pub fn as_closure(&self) -> Option<&ExprClosure> {
        if let Self::Closure(v) = self { Some(v) } else { None }
    }

    pub fn as_const(&self) -> Option<&ExprConst> {
        if let Self::Const(v) = self { Some(v) } else { None }
    }

    pub fn as_continue(&self) -> Option<&ExprContinue> {
        if let Self::Continue(v) = self { Some(v) } else { None }
    }

    pub fn as_for_loop(&self) -> Option<&ExprForLoop> {
        if let Self::ForLoop(v) = self { Some(v) } else { None }
    }

    pub fn as_group(&self) -> Option<&ExprGroup> {
        if let Self::Group(v) = self { Some(v) } else { None }
    }

    pub fn as_if_chain(&self) -> Option<&ExprIfChain> {
        if let Self::IfChain(v) = self { Some(v) } else { None }
    }

    pub fn as_infer(&self) -> Option<&ExprInfer> {
        if let Self::Infer(v) = self { Some(v) } else { None }
    }

    pub fn as_let(&self) -> Option<&ExprLet> {
        if let Self::Let(v) = self { Some(v) } else { None }
    }

    pub fn as_lit(&self) -> Option<&ExprLit> {
        if let Self::Lit(v) = self { Some(v) } else { None }
    }

    pub fn as_loop(&self) -> Option<&ExprLoop> {
        if let Self::Loop(v) = self { Some(v) } else { None }
    }

    pub fn as_macro(&self) -> Option<&ExprMacro> {
        if let Self::Macro(v) = self { Some(v) } else { None }
    }

    pub fn as_match(&self) -> Option<&ExprMatch> {
        if let Self::Match(v) = self { Some(v) } else { None }
    }

    pub fn as_paren(&self) -> Option<&ExprParen> {
        if let Self::Paren(v) = self { Some(v) } else { None }
    }

    pub fn as_path(&self) -> Option<&ExprPath> {
        if let Self::Path(v) = self { Some(v) } else { None }
    }

    pub fn as_postfix(&self) -> Option<&ExprPostfix> {
        if let Self::Postfix(v) = self { Some(v) } else { None }
    }

    pub fn as_range(&self) -> Option<&ExprRange> {
        if let Self::Range(v) = self { Some(v) } else { None }
    }

    pub fn as_raw_addr(&self) -> Option<&ExprRawAddr> {
        if let Self::RawAddr(v) = self { Some(v) } else { None }
    }

    pub fn as_reference(&self) -> Option<&ExprReference> {
        if let Self::Reference(v) = self { Some(v) } else { None }
    }

    pub fn as_repeat(&self) -> Option<&ExprRepeat> {
        if let Self::Repeat(v) = self { Some(v) } else { None }
    }

    pub fn as_return(&self) -> Option<&ExprReturn> {
        if let Self::Return(v) = self { Some(v) } else { None }
    }

    pub fn as_struct(&self) -> Option<&ExprStruct> {
        if let Self::Struct(v) = self { Some(v) } else { None }
    }

    pub fn as_try_block(&self) -> Option<&ExprTryBlock> {
        if let Self::TryBlock(v) = self { Some(v) } else { None }
    }

    pub fn as_tuple(&self) -> Option<&ExprTuple> {
        if let Self::Tuple(v) = self { Some(v) } else { None }
    }

    pub fn as_unary(&self) -> Option<&ExprUnary> {
        if let Self::Unary(v) = self { Some(v) } else { None }
    }

    pub fn as_unsafe(&self) -> Option<&ExprUnsafe> {
        if let Self::Unsafe(v) = self { Some(v) } else { None }
    }

    pub fn as_verbatim(&self) -> Option<&TokenStream> {
        if let Self::Verbatim(v) = self { Some(v) } else { None }
    }

    pub fn as_while(&self) -> Option<&ExprWhile> {
        if let Self::While(v) = self { Some(v) } else { None }
    }

    pub fn as_yield(&self) -> Option<&ExprYield> {
        if let Self::Yield(v) = self { Some(v) } else { None }
    }
}

impl Spanner for Expr {
    fn span(&self) -> Span {
        match self {
            Self::Array(v) => v.span(),
            Self::Assign(v) => v.span(),
            Self::Async(v) => v.span(),
            Self::BinaryChain(v) => v.span(),
            Self::Block(v) => v.span(),
            Self::Break(v) => v.span(),
            Self::CastChain(v) => v.span(),
            Self::Closure(v) => v.span(),
            Self::Const(v) => v.span(),
            Self::Continue(v) => v.span(),
            Self::ForLoop(v) => v.span(),
            Self::Group(v) => v.span(),
            Self::IfChain(v) => v.span(),
            Self::Infer(v) => v.span(),
            Self::Let(v) => v.span(),
            Self::Lit(v) => v.span(),
            Self::Loop(v) => v.span(),
            Self::Macro(v) => v.span(),
            Self::Match(v) => v.span(),
            Self::Paren(v) => v.span(),
            Self::Path(v) => v.span(),
            Self::Postfix(v) => v.span(),
            Self::Range(v) => v.span(),
            Self::RawAddr(v) => v.span(),
            Self::Reference(v) => v.span(),
            Self::Repeat(v) => v.span(),
            Self::Return(v) => v.span(),
            Self::Struct(v) => v.span(),
            Self::TryBlock(v) => v.span(),
            Self::Tuple(v) => v.span(),
            Self::Unary(v) => v.span(),
            Self::Unsafe(v) => v.span(),
            Self::While(v) => v.span(),
            Self::Yield(v) => v.span(),
            Self::Verbatim(v) => v.span(),
        }
    }
}

impl ToTokens for Expr {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            Self::Array(v) => v.to_tokens(t),
            Self::Assign(v) => v.to_tokens(t),
            Self::Async(v) => v.to_tokens(t),
            Self::BinaryChain(v) => v.to_tokens(t),
            Self::Block(v) => v.to_tokens(t),
            Self::Break(v) => v.to_tokens(t),
            Self::CastChain(v) => v.to_tokens(t),
            Self::Closure(v) => v.to_tokens(t),
            Self::Const(v) => v.to_tokens(t),
            Self::Continue(v) => v.to_tokens(t),
            Self::ForLoop(v) => v.to_tokens(t),
            Self::Group(v) => v.to_tokens(t),
            Self::IfChain(v) => v.to_tokens(t),
            Self::Infer(v) => v.to_tokens(t),
            Self::Let(v) => v.to_tokens(t),
            Self::Lit(v) => v.to_tokens(t),
            Self::Loop(v) => v.to_tokens(t),
            Self::Macro(v) => v.to_tokens(t),
            Self::Match(v) => v.to_tokens(t),
            Self::Paren(v) => v.to_tokens(t),
            Self::Path(v) => v.to_tokens(t),
            Self::Postfix(v) => v.to_tokens(t),
            Self::Range(v) => v.to_tokens(t),
            Self::RawAddr(v) => v.to_tokens(t),
            Self::Reference(v) => v.to_tokens(t),
            Self::Repeat(v) => v.to_tokens(t),
            Self::Return(v) => v.to_tokens(t),
            Self::Struct(v) => v.to_tokens(t),
            Self::TryBlock(v) => v.to_tokens(t),
            Self::Tuple(v) => v.to_tokens(t),
            Self::Unary(v) => v.to_tokens(t),
            Self::Unsafe(v) => v.to_tokens(t),
            Self::While(v) => v.to_tokens(t),
            Self::Yield(v) => v.to_tokens(t),
            Self::Verbatim(v) => v.to_tokens(t),
        }
    }
}

impl Parse for Expr {
    fn peek(cursor: Cursor<'_>) -> bool {
        peek::expr(cursor, ExprContext::NORMAL)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        parse::expr(parser, ExprContext::NORMAL)
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        skip::expr(cursor, ExprContext::NORMAL)
    }
}

#[derive(Copy, Clone)]
struct ExprContext {
    allow_struct: bool,
}

impl ExprContext {
    const NORMAL: Self = Self { allow_struct: true };

    const EARLY: Self = Self { allow_struct: false };

    // Whether a range ends here without an end operand. Where a struct
    // literal is not allowed, as in `for i in 0.. {}`, a `{` starts the body
    fn is_end(self, cursor: Cursor<'_>) -> bool {
        cursor.is_empty()
            || <Token![,]>::peek(cursor)
            || <Token![;]>::peek(cursor)
            || (!self.allow_struct && cursor.is_delimited(Delim::Brace))
    }
}
